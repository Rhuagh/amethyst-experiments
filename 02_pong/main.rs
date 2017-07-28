extern crate amethyst;
extern crate remawin;
extern crate time;
extern crate cgmath;
extern crate shrev;
extern crate rand;
extern crate collision;

use amethyst::{Application, State, Trans};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::World;
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::ecs::systems::TransformSystem;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use amethyst::config::Config;
use amethyst::WindowEvent;

mod input;
mod input_mapper;
mod comp;
mod coll;
mod event;
mod system;

use comp::*;
use event::*;
use input_mapper::*;
use input::*;
use system::*;

struct Pong;

impl State for Pong {
    fn on_start(&mut self, world : &mut World, assets : &mut AssetManager, pipe : &mut Pipeline) {
        use amethyst::ecs::resources::{Camera, Projection, ScreenDimensions};
        use amethyst::renderer::Layer;
        use amethyst::renderer::pass::{Clear, DrawFlat};

        let layer = Layer::new("main",
                               vec![Clear::new([0.0, 0.0, 0.0, 1.0]),
                                    DrawFlat::new("main", "main")]);
        pipe.layers.push(layer);

        // Setup camera
        let (left_bound, right_bound) = {
            let dim = world.read_resource::<ScreenDimensions>();
            let mut camera = world.write_resource::<Camera>();
            let aspect_ratio = dim.aspect_ratio;
            let eye = [0., 0., 0.1];
            let target = [0., 0., 0.];
            let up = [0., 1., 0.];

            // Get an Orthographic projection
            let proj = Projection::Orthographic {
                left: -1.0 * aspect_ratio,
                right: 1.0 * aspect_ratio,
                bottom: -1.0,
                top: 1.0,
                near: 0.0,
                far: 1.0,
            };

            camera.proj = proj;
            camera.eye = eye;
            camera.target = target;
            camera.up = up;

            match camera.proj {
                Projection::Orthographic {
                    left,
                    right,
                    ..
                } => (left, right),
                _ => (1.0, 1.0),
            }
        };

        world.add_resource::<GameState>(GameState::new());

        assets.register_asset::<Mesh>();
        assets.register_asset::<Texture>();

        assets.load_asset_from_data::<Texture, [f32; 4]>("white", [1.0, 1.0, 1.0, 1.0]);
        let square_verts = gen_rectangle(1.0, 1.0);
        assets.load_asset_from_data::<Mesh, Vec<VertexPosNormal>>("square", square_verts);
        let square = assets
            .create_renderable("square", "white", "white", "white", 1.0)
            .unwrap();

        let ball = Ball::new();
        let mut local = LocalTransform::default();
        local.translation = [ball.position.x, ball.position.y, 0.0];
        local.scale = [ball.radius, ball.radius, 1.0];
        world
            .create_entity()
            .with(square.clone())
            .with(ball)
            .with(local)
            .with(Transform::default())
            .build();

        let plank = Plank::new(Side::Left);
        let mut local = LocalTransform::default();
        local.scale = [plank.dimensions.x, plank.dimensions.y, 1.0];
        match plank.side {
            Side::Left => local.translation = [left_bound + plank.dimensions.x/2., plank.position, 0.0],
            Side::Right => local.translation = [right_bound - plank.dimensions.x/2., plank.position, 0.0],
        };
        world
            .create_entity()
            .with(square.clone())
            .with(plank)
            .with(local)
            .with(Transform::default())
            .build();

        // Create right plank entity
        let plank = Plank::new(Side::Right);
        let mut local = LocalTransform::default();
        local.scale = [plank.dimensions.x, plank.dimensions.y, 1.0];
        match plank.side {
            Side::Left => local.translation = [left_bound + plank.dimensions.x/2., plank.position, 0.0],
            Side::Right => local.translation = [right_bound - plank.dimensions.x/2., plank.position, 0.0],
        };
        world
            .create_entity()
            .with(square.clone())
            .with(plank)
            .with(local)
            .with(Transform::default())
            .build();

    }

    fn handle_events(&mut self,
                     events: &[WindowEvent],
                     world: &mut World,
                     _: &mut AssetManager,
                     _: &mut Pipeline)
                     -> Trans {
        let mut input = world.write_resource::<AmethystEventMapper<Action, InputContext>>();
        let mut event_handler = world.write_resource::<shrev::EventHandler>();
        for me in input.process(&events.to_vec()) {
            match me {
                remawin::Event::Controller(remawin::ControllerEvent::Action(Action::Exit, _)) |
                remawin::Event::Window(remawin::WindowEvent::Close) => {
                    return Trans::Quit;
                },
                remawin::Event::Window(event) => {
                    event_handler.write_single(IWindowEvent::new(event)).expect("Failed writing event to handler");
                },
                remawin::Event::Controller(event) => {
                    event_handler.write_single(ControllerEvent::new(event)).expect("Failed writing event to handler");
                }
            }
        }
        Trans::None
    }
}


fn main() {
    let path = format!("{}/02_pong/resources/config.yml",
                       env!("CARGO_MANIFEST_DIR"));
    let input_path = format!("{}/02_pong/resources/bindings.yml",
                       env!("CARGO_MANIFEST_DIR"));
    let cfg = DisplayConfig::load(path);
    let dim = cfg.dimensions.as_ref().unwrap().clone();
    let mut game = Application::build(Pong, cfg)
        .register::<Ball>()
        .register::<Plank>()
        .with::<PongSystem>(PongSystem::new(), "pong_system", &[])
        .with::<TransformSystem>(TransformSystem::new(), "transform_system", &["pong_system"])
        .done();
    game.world_mut().add_resource(init_input_system((dim.0 as f64, dim.1 as f64), &input_path));
    game.world_mut().add_resource(init_event_system());
    game.run();
}

fn gen_rectangle(w: f32, h: f32) -> Vec<VertexPosNormal> {
    let data: Vec<VertexPosNormal> = vec![
        VertexPosNormal {
            pos: [-w / 2., -h / 2., 0.],
            normal: [0., 0., 1.],
            tex_coord: [0., 0.],
        },
        VertexPosNormal {
            pos: [w / 2., -h / 2., 0.],
            normal: [0., 0., 1.],
            tex_coord: [1., 0.],
        },
        VertexPosNormal {
            pos: [w / 2., h / 2., 0.],
            normal: [0., 0., 1.],
            tex_coord: [1., 1.],
        },
        VertexPosNormal {
            pos: [w / 2., h / 2., 0.],
            normal: [0., 0., 1.],
            tex_coord: [1., 1.],
        },
        VertexPosNormal {
            pos: [-w / 2., h / 2., 0.],
            normal: [0., 0., 1.],
            tex_coord: [1., 1.],
        },
        VertexPosNormal {
            pos: [-w / 2., -h / 2., 0.],
            normal: [0., 0., 1.],
            tex_coord: [1., 1.],
        }];
    data
}