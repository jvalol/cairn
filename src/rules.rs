//! A run: the tower, what has come out of it, and when it is over. Spec 0001.

use crate::tower::{self, ACROSS, HALF};
use blitzkit::collision::Aabb;
use blitzkit::physics::{Body, Shape, Solver};
use glam::{vec3, Vec3};

/// How fast a block is drawn out, in units a second, and how hard it is pulled
/// to get there, in units a second a second.
///
/// The hand is strong but not infinitely so. Setting the speed outright made the
/// block unstoppable, and a neighbour wedged against one of those leaves at
/// whatever speed the solver needs to get it out of the way: measured, one went
/// thirty three units.
///
/// How hard to pull, then, measured by drawing one block out of a full bottom
/// level and watching how far the worst of the others moved:
///
/// ```text
///       seat 0          seat 1    seat 2    seat 3
///  60   6.7s, 0.30      3.5s      2.9s      2.6s, 0.13
///  80   2.6s, 0.19      0.9s      1.0s      3.4s, 0.42
/// 100   1.6s, 0.16      0.9s      0.9s      1.2s, 39.70, fatal
/// 120   1.2s, 20.32     0.9s      0.9s      1.0s, 0.60
/// 150   0.9s, 35.00     0.9s      0.9s      0.9s, 87.11, fatal
/// ```
///
/// Eighty is the last one where nothing is ever launched, and it gives the game
/// something as well: an inner block slips out in a second and an outer one,
/// which is holding up the edge of everything above it, takes three and lets you
/// watch it resist.
///
/// Past a hundred a pulled block can send a neighbour thirty units or more,
/// which is the engine rather than the game. A driven body forced through a
/// loaded contact is a case blitzkit has not been asked for before.
pub const PULL_SPEED: f32 = 5.0;
pub const PULL_PULL: f32 = 80.0;

/// How far a block's middle has to get from the tower's middle before it counts
/// as out: its own half length, plus half the width of a level, plus a little.
pub const CLEAR: f32 = HALF.x + HALF.x + 0.2;

/// How far the top of it may drop before the run is over. Two levels: one is
/// exactly the height of the block just put on top, so losing only that block
/// would end a run, and losing one block is not a tower coming down.
pub const SURVIVES: f32 = HALF.y * 4.0;

pub const GRAVITY: Vec3 = vec3(0.0, -9.81, 0.0);

/// How wide the ground is. Nothing rests on its edges; it is there so a block
/// that has been let go has somewhere to land.
pub const GROUND: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Phase {
    /// Nothing is moving and a block may be chosen.
    Choosing,
    /// One is being drawn out.
    Pulling,
    /// It came out, and the tower is being left to answer for it.
    Settling,
    Over,
}

pub struct Run {
    blocks: Vec<Body>,
    /// Which level each block belongs to. Blocks are never removed, only moved
    /// to the top, so this stays alongside them and no index ever shifts.
    seated: Vec<usize>,
    ground: Vec<Aabb>,
    solver: Solver,
    /// The block being drawn out, and the way it is going.
    drawing: Option<(usize, Vec3)>,
    phase: Phase,
    /// The highest level that holds any block, and how many of its seats are
    /// taken.
    top: usize,
    filled: usize,
    out: u32,
    /// The highest the tower has stood, to measure a collapse against.
    stood: f32,
}

impl Default for Run {
    fn default() -> Self {
        Self::new()
    }
}

impl Run {
    pub fn new() -> Self {
        let blocks = tower::built();
        let seated = (0..blocks.len()).map(|n| n / ACROSS).collect();

        Self {
            blocks,
            seated,
            ground: vec![Aabb::from_center_size(
                vec3(0.0, -1.0, 0.0),
                vec3(GROUND, 2.0, GROUND),
            )],
            solver: Solver::new(),
            drawing: None,
            phase: Phase::Choosing,
            top: tower::LEVELS - 1,
            filled: ACROSS,
            out: 0,
            stood: tower::LEVELS as f32 * HALF.y * 2.0,
        }
    }

    pub fn blocks(&self) -> &[Body] {
        &self.blocks
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn out(&self) -> u32 {
        self.out
    }

    pub fn drawing(&self) -> Option<usize> {
        self.drawing.map(|(which, _)| which)
    }

    /// Whether a block may be taken. The top level is not a source, nor the one
    /// below it, which is what stops the game being about taking back the block
    /// just put down.
    pub fn may_take(&self, which: usize) -> bool {
        which < self.blocks.len() && self.seated[which] + 1 < self.top
    }

    /// Starts drawing one out, towards whichever end of it `towards` points at.
    pub fn take(&mut self, which: usize, towards: Vec3) {
        if self.phase != Phase::Choosing || !self.may_take(which) {
            return;
        }

        let Some(along) = self.along(which) else {
            return;
        };
        let way = if along.dot(towards) < 0.0 {
            -along
        } else {
            along
        };

        self.blocks[which].wake();
        self.drawing = Some((which, way));
        self.phase = Phase::Pulling;
    }

    /// Which way a block's own length runs, in the world.
    fn along(&self, which: usize) -> Option<Vec3> {
        let Shape::Block { half } = self.blocks[which].shape else {
            return None;
        };

        let longest = if half.x >= half.z { Vec3::X } else { Vec3::Z };

        Some((self.blocks[which].orientation * longest).normalize_or_zero())
    }

    pub fn step(&mut self, dt: f32) {
        // A run that is over goes on being stepped. Returning here froze the
        // tower at the instant it was declared down, which is the instant
        // before any of it has actually fallen: the one thing a player wants to
        // watch, stopped dead on the frame it started.
        if let Some((which, way)) = self.drawing {
            // Drawn rather than struck, and only along its own length: whatever
            // the tower is doing to it across that is left alone, so a block
            // being leaned on can still be pushed about while it comes out.
            let block = &mut self.blocks[which];
            block.wake();

            let along = block.velocity.dot(way);
            let more = (PULL_SPEED - along).clamp(-PULL_PULL * dt, PULL_PULL * dt);
            block.velocity += way * more;
        }

        self.solver
            .step(&mut self.blocks, &self.ground, GRAVITY, dt);

        if let Some((which, _)) = self.drawing {
            let at = self.blocks[which].position;
            if vec3(at.x, 0.0, at.z).length() > CLEAR {
                self.put_on_top(which);
            }
        }

        self.watch();
    }

    /// Puts a block that has come out onto the top of the tower, turned the way
    /// that level runs. A full top level starts another.
    fn put_on_top(&mut self, which: usize) {
        if self.filled >= ACROSS {
            self.top += 1;
            self.filled = 0;
        }

        let (at, flat) = tower::seat(self.top, self.filled);
        let half = if flat {
            HALF
        } else {
            vec3(HALF.z, HALF.y, HALF.x)
        };

        let block = &mut self.blocks[which];
        block.position = at;
        block.orientation = glam::Quat::IDENTITY;
        block.velocity = Vec3::ZERO;
        block.spin = Vec3::ZERO;
        block.shape = Shape::Block { half };
        block.wake();

        self.seated[which] = self.top;
        self.filled += 1;
        self.out += 1;
        self.drawing = None;
        self.phase = Phase::Settling;

        // a contact is remembered by the pair it is between, and one of this
        // pair is somewhere else entirely now
        self.solver.forget();
    }

    /// Watches the top of the tower, and the run ends when it falls.
    fn watch(&mut self) {
        let drawing = self.drawing.map(|(which, _)| which);
        let top = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(n, _)| Some(*n) != drawing)
            .map(|(_, block)| block.position.y + HALF.y)
            .fold(0.0f32, f32::max);

        if self.phase == Phase::Over {
            return;
        }

        self.stood = self.stood.max(top);

        if top < self.stood - SURVIVES {
            self.phase = Phase::Over;
            self.drawing = None;
            return;
        }

        if self.phase == Phase::Settling && self.blocks.iter().all(|block| block.asleep) {
            self.phase = Phase::Choosing;
        }
    }

    /// How tall it stands now, in levels.
    pub fn levels(&self) -> usize {
        tower::levels(&self.blocks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settle(run: &mut Run, ticks: usize) {
        for _ in 0..ticks {
            run.step(1.0 / 120.0);
        }
    }

    /// An inner seat on a low level, which is the one a tower can spare. Two
    /// from the same level is half of it, and a tower does not survive that:
    /// these tests took blocks 0, 1 and 2 to begin with and the second one
    /// brought the whole thing down, correctly.
    fn sparing(level: usize) -> usize {
        level * ACROSS + 1
    }

    /// The same, alternating which of the two inner seats, so a run of takes
    /// does not punch a line of holes down one side of the tower. Taking seat
    /// one from every other level brought it down on the fourth.
    fn evenly(level: usize) -> usize {
        level * ACROSS + if level.is_multiple_of(4) { 1 } else { 2 }
    }

    #[test]
    fn a_new_tower_stands() {
        let mut run = Run::new();
        settle(&mut run, 600);

        assert_eq!(run.levels(), tower::LEVELS, "it came down on its own");
        assert_eq!(run.phase(), Phase::Choosing);
        assert_eq!(run.out(), 0);
    }

    #[test]
    fn the_top_two_are_safe() {
        let run = Run::new();
        let top = tower::LEVELS - 1;

        for seat in 0..ACROSS {
            assert!(!run.may_take(top * ACROSS + seat), "the top level went");
            assert!(
                !run.may_take((top - 1) * ACROSS + seat),
                "the one below the top went"
            );
        }
    }

    #[test]
    fn anything_lower_can_go() {
        let run = Run::new();

        for level in 0..(tower::LEVELS - 2) {
            for seat in 0..ACROSS {
                assert!(
                    run.may_take(level * ACROSS + seat),
                    "level {} seat {} was refused",
                    level,
                    seat
                );
            }
        }
    }

    #[test]
    fn what_comes_out_goes_on_top() {
        let mut run = Run::new();
        settle(&mut run, 600);

        let was = run.levels();
        run.take(sparing(0), Vec3::X);
        settle(&mut run, 900);

        assert_eq!(run.out(), 1, "nothing came out");
        // it went onto the level above the one that was top, which was full
        assert_eq!(run.levels(), was + 1, "it did not go on top");

        // and turned the way that level runs
        let (_, flat) = tower::seat(tower::LEVELS, 0);
        let Shape::Block { half } = run.blocks()[sparing(0)].shape else {
            unreachable!()
        };
        assert_eq!(half.x >= half.z, flat, "it went on sideways");
    }

    #[test]
    fn a_full_level_starts_another() {
        let mut run = Run::new();
        // the tower is built with every level full, so the first one out starts
        // a new level and the next three fill it
        settle(&mut run, 600);

        let mut tops = Vec::new();
        for level in [0usize, 2, 4, 6, 8] {
            run.take(evenly(level), Vec3::X);
            settle(&mut run, 900);
            tops.push(run.top);
        }

        assert_eq!(
            tops,
            vec![
                tower::LEVELS,
                tower::LEVELS,
                tower::LEVELS,
                tower::LEVELS,
                tower::LEVELS + 1
            ],
            "a level took the wrong number of blocks"
        );
    }

    #[test]
    fn the_count_is_what_came_out() {
        let mut run = Run::new();
        settle(&mut run, 600);

        for (n, level) in [0usize, 2, 4].iter().enumerate() {
            assert_eq!(run.out(), n as u32);
            run.take(sparing(*level), Vec3::X);
            settle(&mut run, 900);
        }

        assert_eq!(run.out(), 3);
    }

    #[test]
    fn a_drop_ends_it() {
        let mut run = Run::new();
        settle(&mut run, 600);

        // take a whole level out from under it, which nothing survives
        for seat in 0..ACROSS {
            run.take(seat, Vec3::X);
            settle(&mut run, 600);
        }
        settle(&mut run, 900);

        assert_eq!(run.phase(), Phase::Over, "it stood on nothing");
    }

    #[test]
    fn a_dropped_block_is_not_a_collapse() {
        let mut run = Run::new();
        settle(&mut run, 600);

        // one block out and onto the floor beside the tower, which happens on
        // purpose every turn and must not end anything
        run.take(sparing(0), Vec3::X);
        settle(&mut run, 900);

        assert_ne!(run.phase(), Phase::Over, "one block out ended the run");
        assert_eq!(run.levels(), tower::LEVELS + 1);
    }

    #[test]
    fn the_same_run_twice_is_the_same_run() {
        let play = || {
            let mut run = Run::new();
            settle(&mut run, 600);
            for level in [0usize, 2, 4] {
                run.take(sparing(level), Vec3::X);
                settle(&mut run, 900);
            }
            run
        };

        let (one, other) = (play(), play());

        assert_eq!(one.out(), other.out());
        for (a, b) in one.blocks().iter().zip(other.blocks()) {
            assert_eq!(a.position, b.position);
            assert_eq!(a.orientation, b.orientation);
        }
    }
}
