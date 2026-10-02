//! Holding a block: where a drag asks it to go, and where the camera looks.
//! Spec 0002.
//!
//! The arithmetic lives here rather than in the window so it can be checked
//! without one.

use glam::{Mat4, Vec2, Vec3};

/// How far a block is asked to move for one pixel of drag. A block has to
/// travel its own length to come clear, and a drag of about three hundred
/// pixels should do it.
pub const PER_PIXEL: f32 = 0.014;

/// How far the camera may tilt, in radians. Short of overhead and short of the
/// floor: neither tells you anything and both look wrong.
pub const LOWEST: f32 = 0.08;
pub const HIGHEST: f32 = 1.2;

/// Which way a direction in the world runs on the screen, as a unit vector in
/// pixels, or nothing if it is pointing at or away from the eye.
///
/// Screen y counts down and clip space counts up, which is the sign below and
/// the reason a drag down the window used to pull a block up the tower.
pub fn on_screen(view_projection: Mat4, at: Vec3, way: Vec3) -> Option<Vec2> {
    let flat = |point: Vec3| {
        let clip = view_projection * point.extend(1.0);
        (clip.w.abs() > 1e-4).then(|| Vec2::new(clip.x / clip.w, -clip.y / clip.w))
    };

    let (here, there) = (flat(at)?, flat(at + way)?);
    let moved = there - here;

    (moved.length() > 1e-5).then(|| moved.normalize())
}

/// How far along its own length a block is being asked to move, from how far
/// the cursor has dragged since it was grabbed.
pub fn asked_for(dragged: Vec2, length_on_screen: Vec2) -> f32 {
    dragged.dot(length_on_screen) * PER_PIXEL
}

/// Which way out of the tower a block grabbed at `at` should go: the end that
/// was grabbed.
pub fn grabbed_end(middle: Vec3, at: Vec3, along: Vec3) -> Vec3 {
    if (at - middle).dot(along) < 0.0 {
        -along
    } else {
        along
    }
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

    /// A camera at +z looking at the origin, so world +x runs right across the
    /// screen and world +y runs up it.
    fn looking() -> Mat4 {
        let view = glam::camera::rh::view::look_at_mat4(vec3(0.0, 0.0, 10.0), Vec3::ZERO, Vec3::Y);
        let projection = glam::camera::rh::proj::directx::perspective(1.0, 1.0, 0.1, 100.0);

        projection * view
    }

    #[test]
    fn a_drag_asks_for_that_much() {
        let across = on_screen(looking(), Vec3::ZERO, Vec3::X).expect("x is across the view");
        assert!(across.x > 0.9, "world +x should run right: {}", across);

        // three hundred pixels right is a block's length and then some
        let asked = asked_for(Vec2::new(300.0, 0.0), across);
        assert!(asked > 4.0, "{} is not far enough to clear a block", asked);

        // and dragging the other way asks for the other way
        assert!(asked_for(Vec2::new(-300.0, 0.0), across) < -4.0);
    }

    #[test]
    fn a_drag_across_it_asks_for_nothing() {
        let across = on_screen(looking(), Vec3::ZERO, Vec3::X).expect("x is across the view");

        let asked = asked_for(Vec2::new(0.0, 300.0), across);
        assert!(
            asked.abs() < 0.1,
            "a drag down the screen asked for {}",
            asked
        );
    }

    #[test]
    fn a_direction_pointing_at_the_eye_has_no_screen_way() {
        // straight towards the camera, which is nowhere on the screen
        assert!(on_screen(looking(), Vec3::ZERO, Vec3::ZERO).is_none());
    }

    #[test]
    fn it_comes_out_the_end_that_was_grabbed() {
        let middle = vec3(0.0, 1.0, 0.0);
        let along = Vec3::X;

        // grabbed on the +x half, so it goes +x
        assert_eq!(
            grabbed_end(middle, middle + vec3(1.5, 0.0, 0.0), along),
            Vec3::X
        );
    }

    #[test]
    fn the_other_end_goes_the_other_way() {
        let middle = vec3(0.0, 1.0, 0.0);
        let along = Vec3::X;

        assert_eq!(
            grabbed_end(middle, middle + vec3(-1.5, 0.0, 0.0), along),
            -Vec3::X
        );

        // and the answer does not depend on which way `along` was handed over
        assert_eq!(
            grabbed_end(middle, middle + vec3(-1.5, 0.0, 0.0), -along),
            -Vec3::X
        );
    }

    #[test]
    fn the_tilt_is_clamped() {
        let target = vec3(0.0, 5.0, 0.0);

        // asked for overhead, and it stops short of it
        let high = eye(target, 0.0, 3.0, 10.0);
        assert!(high.y - target.y < 10.0, "it went overhead: {}", high);
        assert!((high.y - target.y - HIGHEST.sin() * 10.0).abs() < 1e-4);

        // asked for underneath, and it stops short of the floor
        let low = eye(target, 0.0, -3.0, 10.0);
        assert!(low.y > target.y, "it went under: {}", low);
        assert!((low.y - target.y - LOWEST.sin() * 10.0).abs() < 1e-4);

        // and it is always the right distance away
        for up in [-3.0f32, 0.0, 0.5, 3.0] {
            assert!(((eye(target, 1.0, up, 10.0) - target).length() - 10.0).abs() < 1e-3);
        }
    }
}
