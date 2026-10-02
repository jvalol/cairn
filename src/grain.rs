//! What a block is made of: wood, drawn rather than loaded. Spec 0003.
//!
//! The same trick as marble's checker and poolhall's stripes. A plank's grain
//! is a few long lines down its length with the colour shifting between them,
//! and that is cheaper to write than to ship.

use blitzkit::texture::TextureData;

/// How big one is. Wide and short, because a block is four long and one across
/// and every face gets the whole texture: the long faces stretch the width over
/// four units and the ends squeeze it into one.
pub const WIDE: u32 = 256;
pub const TALL: u32 = 64;

/// How many different ones there are. A tower of forty eight blocks all wearing
/// the same grain reads as a printed pattern rather than as wood.
pub const KINDS: usize = 6;

/// The lightest and darkest a board gets, as whole bytes rather than fractions
/// because that is what goes into the texture.
const PALE: [f32; 3] = [214.0, 176.0, 128.0];
const DARK: [f32; 3] = [150.0, 112.0, 74.0];

/// A hash, for the speckle between the lines. Not random: the same block wants
/// the same grain every time it is built, or a tower changes its mind about
/// what it is made of when you press space.
fn scattered(x: u32, y: u32, seed: u32) -> f32 {
    let mut n = x
        .wrapping_mul(374_761_393)
        .wrapping_add(y.wrapping_mul(668_265_263))
        .wrapping_add(seed.wrapping_mul(2_246_822_519));
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);

    ((n ^ (n >> 16)) & 0xffff) as f32 / 65535.0
}

/// How dark the wood is at a point, from nothing to all of it.
///
/// Grain runs along the board, so it varies with how far across the board you
/// are and hardly at all with how far along. Three waves of different lengths
/// rather than one, so the lines are not evenly spaced, and a little speckle on
/// top so they are not clean.
fn darkness(u: f32, v: f32, seed: u32) -> f32 {
    // A board's lines are not dead straight, but only just. This was 0.03, and
    // against a line frequency of 73 that is four radians of phase, so the fine
    // lines swung the whole way along the board and the grain ran the wrong
    // way.
    let wander = (u * 6.0 + seed as f32).sin() * 0.004;
    let across = v + wander;

    let lines = (across * 38.0).sin() * 0.5
        + (across * 11.0 + seed as f32 * 1.7).sin() * 0.3
        + (across * 73.0 + seed as f32 * 0.9).sin() * 0.2;

    // Streaky rather than dusty: the speckle is sampled coarsely along the
    // board and finely across it, so it runs in the same direction as the
    // lines. Sampled evenly it varied as much along the board as across and
    // the whole thing read as noise on wood rather than as wood.
    let speckle = scattered(
        (u * WIDE as f32 / 24.0) as u32,
        (v * TALL as f32) as u32,
        seed,
    ) - 0.5;

    ((lines * 0.5 + 0.5) * 0.85 + speckle * 0.15).clamp(0.0, 1.0)
}

/// One board's worth of pixels.
fn board(seed: u32) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((WIDE * TALL * 4) as usize);

    for y in 0..TALL {
        for x in 0..WIDE {
            let u = x as f32 / WIDE as f32;
            let v = y as f32 / TALL as f32;
            let dark = darkness(u, v, seed);

            for channel in 0..3 {
                pixels.push((PALE[channel] + (DARK[channel] - PALE[channel]) * dark) as u8);
            }
            pixels.push(255);
        }
    }

    pixels
}

/// Every kind of board, in order, so a block can take the one its number names.
pub fn boards() -> Vec<TextureData> {
    (0..KINDS)
        .map(|kind| TextureData::from_pixels(WIDE, TALL, board(kind as u32)))
        .collect()
}

/// Which board a block wears. Its own number, so the same block is the same
/// wood every time the tower is built, and neighbours in a level differ.
pub fn worn_by(block: usize) -> usize {
    block % KINDS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_is_one_board_of_each_kind() {
        let boards = boards();

        assert_eq!(boards.len(), KINDS);
        for board in &boards {
            assert_eq!(board.width(), WIDE);
            assert_eq!(board.height(), TALL);
        }
    }

    #[test]
    fn the_kinds_are_different_wood() {
        let one = board(0);
        let other = board(1);

        let apart = one
            .iter()
            .zip(&other)
            .filter(|(a, b)| a.abs_diff(**b) > 8)
            .count();

        assert!(
            apart > one.len() / 10,
            "only {} of {} bytes differ, which is the same board twice",
            apart,
            one.len()
        );
    }

    #[test]
    fn the_same_block_is_always_the_same_wood() {
        assert_eq!(board(3), board(3));
        assert_eq!(worn_by(7), worn_by(7 + KINDS));
        assert_ne!(worn_by(0), worn_by(1), "neighbours in a level match");
    }

    /// Grain runs along a board, so it changes across one and hardly at all
    /// along it. A texture that varied the other way would read as a ladder.
    #[test]
    fn the_grain_runs_along_the_board() {
        let along = (0..40)
            .map(|n| darkness(n as f32 / 40.0, 0.5, 0))
            .collect::<Vec<_>>();
        let across = (0..40)
            .map(|n| darkness(0.5, n as f32 / 40.0, 0))
            .collect::<Vec<_>>();

        let spread = |of: &[f32]| {
            of.iter().fold(0.0f32, |a, b| a.max(*b)) - of.iter().fold(1.0f32, |a, b| a.min(*b))
        };

        assert!(
            spread(&across) > spread(&along) * 1.5,
            "across {} against along {}",
            spread(&across),
            spread(&along)
        );
    }

    #[test]
    fn it_stays_between_pale_and_dark() {
        for seed in 0..KINDS as u32 {
            for pixel in board(seed).chunks(4) {
                assert_eq!(pixel[3], 255, "wood is not see through");
                assert!(pixel[0] >= DARK[0] as u8 - 1 && pixel[0] <= PALE[0] as u8 + 1);
            }
        }
    }
}
