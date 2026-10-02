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

/// Half a block. Square in section and four long, so a level is four by four.
pub const HALF: Vec3 = vec3(2.0, 0.5, 0.5);

/// How bouncy a block is, which is not at all. Wood on wood at the speeds a
/// tower moves at does not come back.
pub const BOUNCE: f32 = 0.0;
/// How much they grip. A number to find rather than to pick: too little and a
/// block cannot be slid out without the tower following it, too much and it
/// cannot be slid out at all.
pub const GRIP: f32 = 0.6;

/// Where the blocks of a level sit, as an offset along the level's own width.
pub fn seats() -> Vec<f32> {
    (0..ACROSS)
        .map(|seat| (seat as f32 - (ACROSS as f32 - 1.0) * 0.5) * HALF.z * 2.0)
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

/// How tall it still reaches, in levels.
pub fn levels(blocks: &[Body]) -> usize {
    let top = blocks
        .iter()
        .map(|block| block.position.y)
        .fold(0.0f32, f32::max);

    (((top - HALF.y) / (HALF.y * 2.0)).round() as usize + 1).min(LEVELS)
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
    fn a_level_is_four_blocks_side_by_side_touching() {
        let seats = seats();

        assert_eq!(seats.len(), ACROSS);
        for pair in seats.windows(2) {
            let apart = pair[1] - pair[0];
            assert!(
                (apart - HALF.z * 2.0).abs() < 1e-6,
                "{} apart, which is not one block",
                apart
            );
        }

        // and the level is as wide as a block is long, so it is square in plan
        let width = seats[ACROSS - 1] - seats[0] + HALF.z * 2.0;
        assert!((width - HALF.x * 2.0).abs() < 1e-6, "{} across", width);
    }
}
