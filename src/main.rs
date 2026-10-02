//! cairn: a tower of blocks you take apart one at a time. See `specs/`.
//!
//! This is the scaffolding the repo was born with: it builds the tower, holds
//! it up, and lets you walk round it. Taking blocks out is spec 0001 and is not
//! built yet.

mod tower;

use blitzkit::camera::Camera;
use blitzkit::collision::Aabb;
use blitzkit::geometry::Geometry;
use blitzkit::keyboard::{KeyboardInput, KeyboardKey, KeyboardKeyState};
use blitzkit::mesh::{MeshData, Transform};
use blitzkit::mouse::{MouseButton, MouseInput};
use blitzkit::physics::{Body, Shape, Solver};
use blitzkit::renderer::render_text::{RenderText, TextRenderer};
use blitzkit::renderer::scene::{MeshId, Scene};
use blitzkit::renderer::Renderer;
use blitzkit::sound::SoundSystem;
use blitzkit::{start, Game};
use glam::{vec2, vec3, vec4, Vec2, Vec3};

const FLOOR: f32 = 24.0;
const GRAVITY: f32 = -9.81;

struct Cairn {
    block_mesh: Option<MeshId>,
    floor_mesh: Option<MeshId>,
    blocks: Vec<Body>,
    ground: Vec<Aabb>,
    /// Kept across frames rather than made fresh each one. Blitzkit's free
    /// `step` remembers nothing and sleeps nothing, and a twelve level stack
    /// sags the moment it is built without the first and leans while the player
    /// is thinking without the second.
    solver: Solver,
    camera_angle: f32,
    turning: bool,
    distance: f32,
    quitting: bool,
}

impl Cairn {
    fn new() -> Self {
        Self {
            block_mesh: None,
            floor_mesh: None,
            blocks: tower::built(),
            ground: vec![Aabb::from_center_size(
                vec3(0.0, -1.0, 0.0),
                vec3(FLOOR, 2.0, FLOOR),
            )],
            solver: Solver::new(),
            camera_angle: 0.6,
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
        self.solver
            .step(&mut self.blocks, &self.ground, vec3(0.0, GRAVITY, 0.0), dt);

        let asleep = self.blocks.iter().filter(|block| block.asleep).count();

        text_renderer.reset();
        for (line, text) in vec![
            String::from("[COPY - Jake] drag with the right button to walk round it, scroll zooms"),
            format!(
                "[COPY - Jake] {} levels of {}, {} of {} settled",
                tower::levels(&self.blocks),
                tower::LEVELS,
                asleep,
                self.blocks.len()
            ),
            String::from("[COPY - Jake] pulling blocks out is spec 0001, and is not built yet"),
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
            &Transform::at(Vec3::ZERO).with_scale(Vec3::splat(FLOOR)),
            vec4(0.17, 0.19, 0.22, 1.0),
        );

        for body in self.blocks.iter() {
            let Shape::Block { half } = body.shape else {
                continue;
            };

            // every other level a shade apart, so which way a level runs is
            // something you can see rather than work out
            let level = (body.position.y / (tower::HALF.y * 2.0)) as usize;
            let colour = if level.is_multiple_of(2) {
                vec4(0.78, 0.62, 0.42, 1.0)
            } else {
                vec4(0.70, 0.54, 0.36, 1.0)
            };

            scene.push_material(
                block,
                &Transform::at(body.position)
                    .with_rotation(body.orientation)
                    .with_scale(half * 2.0),
                colour,
                40.0,
            );
        }

        camera.target = vec3(0.0, tower::LEVELS as f32 * tower::HALF.y, 0.0);
        camera.position = camera.target
            + vec3(
                self.camera_angle.sin() * self.distance,
                self.distance * 0.3,
                self.camera_angle.cos() * self.distance,
            );
    }

    fn process_keyboard(&mut self, input: KeyboardInput) {
        let held = input.state == KeyboardKeyState::Pressed;
        match input.key {
            KeyboardKey::Space if held => {
                self.blocks = tower::built();
                self.solver.forget();
            }
            KeyboardKey::Escape => self.quitting = held,
            _ => (),
        }
    }

    fn process_mouse(&mut self, input: MouseInput) {
        if input.button == MouseButton::Right {
            self.turning = input.is_pressed();
        }
    }

    fn mouse_motion(&mut self, delta: Vec2) {
        if self.turning {
            self.camera_angle += delta.x * 0.005;
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
