//! cairn: a tower of blocks you take apart one at a time. See `specs/`.

mod hand;
mod rules;
mod tower;

use blitzkit::camera::Camera;
use blitzkit::collision::Ray;
use blitzkit::geometry::Geometry;
use blitzkit::keyboard::{KeyboardInput, KeyboardKey, KeyboardKeyState};
use blitzkit::mesh::{MeshData, Transform};
use blitzkit::mouse::{MouseButton, MouseInput};
use blitzkit::physics::Shape;
use blitzkit::renderer::render_text::{RenderText, TextRenderer};
use blitzkit::renderer::scene::{MeshId, Scene};
use blitzkit::renderer::Renderer;
use blitzkit::sound::SoundSystem;
use blitzkit::{start, Game};
use glam::{vec2, vec3, vec4, Quat, Vec2, Vec3};
use rules::{Phase, Run};

/// Where a ray first meets a block: the slab test, in the block's own frame.
fn hit_block(ray: &Ray, middle: Vec3, turn: Quat, half: Vec3) -> Option<f32> {
    let back = turn.inverse();
    let from = back * (ray.origin - middle);
    let way = back * ray.direction;

    let mut entry = f32::NEG_INFINITY;
    let mut exit = f32::INFINITY;

    for n in 0..3 {
        if way[n].abs() < 1e-6 {
            if from[n].abs() > half[n] {
                return None;
            }
            continue;
        }

        let (near, far) = ((-half[n] - from[n]) / way[n], (half[n] - from[n]) / way[n]);
        entry = entry.max(near.min(far));
        exit = exit.min(near.max(far));
    }

    if exit < entry.max(0.0) {
        None
    } else {
        Some(entry.max(0.0))
    }
}

struct Cairn {
    block_mesh: Option<MeshId>,
    floor_mesh: Option<MeshId>,
    run: Run,
    cursor: Vec2,
    /// Which block is under the cursor, and where the camera was looking from,
    /// worked out in `draw` where the camera is.
    picked: Option<usize>,
    /// Where the ray met it, which is the end that was grabbed.
    aimed: Option<Vec3>,
    /// Where the camera is looking, eased rather than set.
    looking: f32,
    /// What the whole world looks like from there, kept so a drag can be
    /// measured along the held block's own length as it lies on the screen.
    seen: glam::Mat4,
    /// Where the cursor was when the block in hand was grabbed, and which way
    /// that block's length runs on the screen.
    grabbed_at: Vec2,
    length_on_screen: Vec2,
    /// Why the last click did nothing, if it did nothing.
    refused: Option<&'static str>,
    camera_angle: f32,
    camera_up: f32,
    turning: bool,
    distance: f32,
    quitting: bool,
}

impl Cairn {
    fn new() -> Self {
        Self {
            block_mesh: None,
            floor_mesh: None,
            run: Run::new(),
            cursor: Vec2::ZERO,
            picked: None,
            aimed: None,
            // The sun travels towards -x and -z, so a camera at +x and +z has
            // the tower's shadow directly behind it and the floor reads as
            // empty. From round here it lies across the view.
            looking: tower::LEVELS as f32 * tower::HALF.y * 0.9,
            seen: glam::Mat4::IDENTITY,
            grabbed_at: Vec2::ZERO,
            length_on_screen: Vec2::X,
            refused: None,
            camera_angle: 2.5,
            camera_up: 0.35,
            turning: false,
            distance: 18.0,
            quitting: false,
        }
    }
}

impl Game for Cairn {
    fn load(&mut self, renderer: &mut Renderer) {
        self.block_mesh = Some(renderer.add_mesh(&MeshData::cube()));
        self.floor_mesh = Some(renderer.add_mesh(&MeshData::plane()));
    }

    fn initialize(
        &mut self,
        _geometry: &mut Geometry,
        _text_renderer: &mut TextRenderer,
        _sound_system: &SoundSystem,
        _window_size: (f32, f32),
    ) {
    }

    fn update(
        &mut self,
        dt: f32,
        _geometry: &mut Geometry,
        text_renderer: &mut TextRenderer,
        _sound_system: &SoundSystem,
    ) {
        self.run.step(dt);

        let saying = match self.run.phase() {
            Phase::Choosing => match self.refused {
                Some(why) => String::from(why),
                None => String::from("press on a block and drag to draw it out"),
            },
            Phase::Pulling => match self.run.slack() {
                // the drag has gone somewhere the block has not, which is what
                // a block that will not come feels like
                Some((came, asked)) if asked - came > 0.5 => {
                    String::from("it is not coming. something is on it")
                }
                _ => String::from("drawing it out. let go to leave it there"),
            },
            Phase::Settling => String::from("waiting to see"),
            Phase::Over => format!(
                "it came down. {} out. space to build it again",
                self.run.out()
            ),
        };

        text_renderer.reset();
        for (line, text) in vec![
            format!(
                "{} out, standing {} levels",
                self.run.out(),
                self.run.levels()
            ),
            saying,
            String::from("right-drag turns and tilts, scroll zooms"),
        ]
        .into_iter()
        .enumerate()
        {
            text_renderer.push_render_text(RenderText {
                position: vec2(20.0, 20.0 + line as f32 * 24.0),
                text,
                size: 14.0,
                ..Default::default()
            });
        }
    }

    fn draw(&mut self, scene: &mut Scene, camera: &mut Camera) {
        let (block, floor) = match (self.block_mesh, self.floor_mesh) {
            (Some(block), Some(floor)) => (block, floor),
            _ => return,
        };

        scene.push_colored(
            floor,
            &Transform::at(Vec3::ZERO).with_scale(Vec3::splat(rules::GROUND)),
            vec4(0.17, 0.19, 0.22, 1.0),
        );

        let drawing = self.run.held();
        for (which, body) in self.run.blocks().iter().enumerate() {
            let Shape::Block { half } = body.shape else {
                continue;
            };

            // every other level a shade apart, so which way a level runs is
            // something you can see rather than work out
            let level = (body.position.y / (tower::HALF.y * 2.0)) as usize;
            let mut colour = if level.is_multiple_of(2) {
                vec4(0.78, 0.62, 0.42, 1.0)
            } else {
                vec4(0.70, 0.54, 0.36, 1.0)
            };

            if drawing == Some(which) {
                colour = vec4(0.95, 0.45, 0.25, 1.0);
            } else if self.picked == Some(which) && self.run.may_take(which) {
                colour = (colour + vec4(0.22, 0.22, 0.22, 0.0)).min(vec4(1.0, 1.0, 1.0, 1.0));
            }

            scene.push_material(
                block,
                &Transform::at(body.position)
                    .with_rotation(body.orientation)
                    .with_scale(half * 2.0),
                colour,
                40.0,
            );
        }

        // Eased towards where the tower is rather than set to it. Read
        // straight, the height jumps about by whole levels while a tower is
        // coming down and takes the camera with it.
        let want = self.run.levels() as f32 * tower::HALF.y * 0.9;
        self.looking += (want - self.looking) * 0.02;
        camera.target = vec3(0.0, self.looking, 0.0);
        camera.position = hand::eye(
            camera.target,
            self.camera_angle,
            self.camera_up,
            self.distance,
        );
        self.seen = camera.view_projection();

        let ray = camera.ray_through(self.cursor);
        self.picked =
            self.run
                .blocks()
                .iter()
                .enumerate()
                .filter_map(|(which, body)| match body.shape {
                    Shape::Block { half } => hit_block(&ray, body.position, body.orientation, half)
                        .map(|away| (away, which)),
                    Shape::Sphere { .. } => None,
                })
                .min_by(|one, other| one.0.total_cmp(&other.0))
                .map(|(_, which)| which);
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        let held = input.state == KeyboardKeyState::Pressed;
        match input.key {
            KeyboardKey::Space if held => {
                self.run = Run::new();
                self.refused = None;
            }
            KeyboardKey::Escape => self.quitting = held,
            _ => (),
        }
    }

    fn process_mouse(&mut self, input: MouseInput) {
        match input.button {
            MouseButton::Right => self.turning = input.is_pressed(),
            MouseButton::Left if input.is_pressed() => {
                self.refused = None;

                let Some(which) = self.picked else {
                    return;
                };
                if !self.run.may_take(which) {
                    self.refused = Some("that one is holding up the top. try lower down");
                    return;
                }

                let body = self.run.blocks()[which];
                let Some(along) = self.run.along(which) else {
                    return;
                };
                let way =
                    hand::grabbed_end(body.position, self.aimed.unwrap_or(body.position), along);

                self.grabbed_at = self.cursor;
                self.length_on_screen =
                    hand::on_screen(self.seen, body.position, way).unwrap_or(Vec2::X);
                self.run.grab(which, way);
            }
            MouseButton::Left => self.run.let_go(),
            _ => (),
        }
    }

    fn cursor_moved(&mut self, position: Vec2) {
        self.cursor = position;

        if self.run.held().is_some() {
            self.run.ask_for(hand::asked_for(
                position - self.grabbed_at,
                self.length_on_screen,
            ));
        }
    }

    fn mouse_motion(&mut self, delta: Vec2) {
        if self.turning {
            self.camera_angle += delta.x * 0.005;
            self.camera_up = (self.camera_up - delta.y * 0.004).clamp(hand::LOWEST, hand::HIGHEST);
        }
    }

    fn mouse_wheel(&mut self, delta: Vec2) {
        self.distance = (self.distance - delta.y * 0.05).clamp(7.0, 34.0);
    }

    fn is_quitting(&self) -> bool {
        self.quitting
    }

    fn focus_changed(&mut self, _focus: bool) {}
}

fn main() {
    start("cairn", Box::new(Cairn::new()));
}
