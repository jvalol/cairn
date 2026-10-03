//! What a cairn is made of, per spec 0001.

use blitzkit::physics::Body;
use glam::{vec3, Vec3};

/// Twelve of them, which with four to a level is forty eight blocks. Not the
/// fifty four, three to a level and eighteen levels of the game this will be
/// compared to, which are a registered mark together.
pub const LEVELS: usize = 12;
/// Four to a level, because three is a see-saw: take one of three and the level
/// above rests on two with a gap where the middle was. Take one of four and
/// three remain, two of them still under the level above.
pub const ACROSS: usize = 4;

/// How long a block is, which is also how wide a level is: four blocks and the
/// gaps between them come to exactly this, so every level is square in plan and
/// the one above it lands square on it.
pub const LONG: f32 = 4.0;

/// The gap between two blocks of a level. Real ones are cut a hair narrow and
/// it matters twice over. A level with no gaps reads as one slab, so a block
/// drawn out leaves a hole nobody can see. And blocks pressed against each
/// other are held by their sides as well as from above, which is grip the game
/// should not have.
pub const GAP: f32 = 0.08;

/// Half a block. Square in section, so its width and its thickness are the same
/// number, and that number is whatever is left of a level once the gaps are
/// taken out.
pub const WIDE: f32 = (LONG - (ACROSS as f32 - 1.0) * GAP) / ACROSS as f32;
pub const HALF: Vec3 = vec3(LONG * 0.5, WIDE * 0.5, WIDE * 0.5);

/// How bouncy a block is, which is not at all. Wood on wood at the speeds a
/// tower moves at does not come back.
pub const BOUNCE: f32 = 0.0;
/// How much they grip. Found rather than picked, per spec 0001. Drawing one
/// block out of a full bottom level and measuring how far the worst of the
/// other forty seven moved, and whether the run survived:
///
/// ```text
///        seat 0        seat 1   seat 2        seat 3
/// 0.25   0.47          0.10     0.42          5.62, fatal
/// 0.40   0.34          0.18     0.33          0.73
/// 0.60   3.06, fatal   0.16     0.13          0.60
/// ```
///
/// 0.4 is the only one where a full level can spare any of its four, which is
/// what the first pull of a game should be. Too little and an outer block slides
/// before the ones above it have settled onto what is left; too much and drawing
/// one out drags its neighbours with it.
///
/// These numbers are from after the gap arrived. Before it, blocks were pressed
/// against each other side to side and an outer one was fatal at every grip
/// tried, which looked like a game about which seat to pick and was really a
/// game about a gap that was missing.
pub const GRIP: f32 = 0.4;

/// Where the blocks of a level sit, as an offset along the level's own width.
pub fn seats() -> Vec<f32> {
    (0..ACROSS)
        .map(|seat| (seat as f32 - (ACROSS as f32 - 1.0) * 0.5) * (WIDE + GAP))
        .collect()
}

/// Where one block of one level goes, and which way it faces.
pub fn seat(level: usize, seat: usize) -> (Vec3, bool) {
    let along = seats()[seat];
    let height = HALF.y + level as f32 * HALF.y * 2.0;
    // every other level runs the other way, which is what ties them together
    let flat = level.is_multiple_of(2);

    let at = if flat {
        vec3(0.0, height, along)
    } else {
        vec3(along, height, 0.0)
    };

    (at, flat)
}

/// A whole cairn, built and not yet settled.
pub fn built() -> Vec<Body> {
    let mut blocks = Vec::new();

    for level in 0..LEVELS {
        for which in 0..ACROSS {
            let (at, flat) = seat(level, which);
            let half = if flat {
                HALF
            } else {
                vec3(HALF.z, HALF.y, HALF.x)
            };

            blocks.push(
                Body::block(at, half, 1.0)
                    .with_restitution(BOUNCE)
                    .with_friction(GRIP),
            );
        }
    }

    blocks
}

/// How tall it reaches, in levels. Not capped at `LEVELS`: a run puts what it
/// pulls on top, so a tower that is going well is taller than it was built.
pub fn levels(blocks: &[Body]) -> usize {
    let top = blocks
        .iter()
        .map(|block| block.position.y)
        .fold(0.0f32, f32::max);

    ((top - HALF.y) / (HALF.y * 2.0)).round().max(0.0) as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_tower_is_twelve_of_four() {
        let blocks = built();

        assert_eq!(blocks.len(), LEVELS * ACROSS);
        assert_eq!(blocks.len(), 48);
        assert_eq!(levels(&blocks), LEVELS);
    }

    #[test]
    fn every_level_crosses_the_one_below() {
        // a level running along z is four wide in z and four long in x, and the
        // one above it is the other way round
        let (_, flat) = seat(0, 0);
        let (_, next) = seat(1, 0);

        assert!(flat != next, "two levels running the same way");

        for level in 0..LEVELS {
            let (_, this) = seat(level, 0);
            assert_eq!(this, level.is_multiple_of(2));
        }
    }

    #[test]
    fn a_level_is_four_blocks_side_by_side_with_a_gap() {
        let seats = seats();

        assert_eq!(seats.len(), ACROSS);
        for pair in seats.windows(2) {
            let apart = pair[1] - pair[0] - WIDE;
            assert!(
                (apart - GAP).abs() < 1e-6,
                "{} of gap, which is not what was asked for",
                apart
            );
        }

        // and the level is as wide as a block is long, so it is square in plan
        // and the level above lands square on it
        let width = seats[ACROSS - 1] - seats[0] + WIDE;
        assert!(
            (width - LONG).abs() < 1e-6,
            "{} across, not {}",
            width,
            LONG
        );

        // and a block is square in section
        assert!((HALF.y - HALF.z).abs() < 1e-6);
    }
}
