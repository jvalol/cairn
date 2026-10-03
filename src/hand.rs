//! Holding a block: where the cursor is asking it to go, and where the camera
//! looks from. Spec 0002.
//!
//! The arithmetic lives here rather than in the window so it can be checked
//! without one.

use glam::Vec3;

/// How far the camera may tilt, in radians. Short of overhead and short of the
/// floor: neither tells you anything and both look wrong.
pub const LOWEST: f32 = 0.08;
pub const HIGHEST: f32 = 1.2;

/// Where a ray comes closest to a line, as a distance along that line from its
/// own point.
///
/// This is how a held block follows the cursor. The cursor points somewhere,
/// and the place on the block's own length nearest to what it is pointing at is
/// where the block is asked to be. Nothing is measured in pixels, so it does not
/// matter how the block's length happens to lie on the screen.
///
/// It was pixels, dragged along that length as it appeared on screen. The
/// trouble is a block pointing away from the eye: its length is a few pixels
/// however long the block is, the direction is noise, and any drag asks for
/// almost nothing. What that looked like was blocks left hanging half out of
/// the tower because finishing the pull had become impossible.
///
/// Nothing comes back when the line runs along the ray, which is the one case
/// with no answer rather than a bad one: the cursor is pointing at the whole
/// length at once.
pub fn along_the_line(from: Vec3, towards: Vec3, on: Vec3, way: Vec3) -> Option<f32> {
    let (ray, line) = (towards.normalize_or_zero(), way.normalize_or_zero());
    let between = line.dot(ray);
    let apart = 1.0 - between * between;

    if apart < 1e-4 {
        return None;
    }

    let gap = on - from;

    Some((between * ray.dot(gap) - line.dot(gap)) / apart)
}

/// Where the camera sits, given how far round and how far up it has been
/// dragged.
pub fn eye(target: Vec3, round: f32, up: f32, away: f32) -> Vec3 {
    let up = up.clamp(LOWEST, HIGHEST);

    target
        + Vec3::new(
            round.sin() * up.cos() * away,
            up.sin() * away,
            round.cos() * up.cos() * away,
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec3;

    /// Spec 0002: the cursor points somewhere, and the block goes to the place
    /// on its own length nearest to that.
    #[test]
    fn the_block_goes_where_it_is_pointed() {
        let eye = vec3(0.0, 0.0, 10.0);
        let block = Vec3::ZERO;

        let asked = |aimed: Vec3| {
            along_the_line(eye, aimed - eye, block, Vec3::X).expect("across the view")
        };

        assert!(asked(block).abs() < 1e-4, "{}", asked(block));
        assert!((asked(vec3(2.0, 0.0, 0.0)) - 2.0).abs() < 1e-3);
        assert!((asked(vec3(-2.0, 0.0, 0.0)) + 2.0).abs() < 1e-3);
    }

    /// And it works for a block pointing away from the eye, which is the case
    /// the pixel version could not do at all.
    #[test]
    fn a_block_pointing_away_still_follows() {
        let eye = vec3(0.0, 6.0, 10.0);
        let aimed = vec3(0.0, 0.0, -3.0);

        let at =
            along_the_line(eye, aimed - eye, Vec3::ZERO, Vec3::Z).expect("not quite along the ray");

        assert!((at + 3.0).abs() < 0.3, "it asked for {} rather than -3", at);
    }

    /// The answer does not depend on which way round the length was handed
    /// over, beyond its sign, so where a block was grabbed decides nothing.
    #[test]
    fn which_way_the_length_was_given_only_flips_the_sign() {
        let eye = vec3(0.0, 0.0, 10.0);
        let aimed = vec3(2.0, 0.0, 0.0);

        let one = along_the_line(eye, aimed - eye, Vec3::ZERO, Vec3::X).unwrap();
        let other = along_the_line(eye, aimed - eye, Vec3::ZERO, -Vec3::X).unwrap();

        assert!((one + other).abs() < 1e-4, "{} against {}", one, other);
    }

    /// Except dead along the ray, which has no answer rather than a bad one.
    #[test]
    fn a_block_down_the_ray_has_no_answer() {
        assert!(along_the_line(vec3(0.0, 0.0, 10.0), -Vec3::Z, Vec3::ZERO, Vec3::Z).is_none());
    }

    #[test]
    fn the_tilt_is_clamped() {
        let target = vec3(0.0, 5.0, 0.0);

        let high = eye(target, 0.0, 3.0, 10.0);
        assert!((high.y - target.y - HIGHEST.sin() * 10.0).abs() < 1e-4);

        let low = eye(target, 0.0, -3.0, 10.0);
        assert!(low.y > target.y, "it went under: {}", low);
        assert!((low.y - target.y - LOWEST.sin() * 10.0).abs() < 1e-4);

        for up in [-3.0f32, 0.0, 0.5, 3.0] {
            assert!(((eye(target, 1.0, up, 10.0) - target).length() - 10.0).abs() < 1e-3);
        }
    }
}
