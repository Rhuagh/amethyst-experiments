extern crate amethyst;
extern crate remawin;
extern crate time;
extern crate cgmath;
extern crate shrev;
extern crate rand;
extern crate collision;

use rand::Rng;

use cgmath::InnerSpace;
use collision::Intersect;

use amethyst::{Application, State, Trans};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::{Component, Fetch, FetchMut, Join, System, VecStorage, World, WriteStorage};
use amethyst::ecs::components::{Mesh, LocalTransform, Texture, Transform};
use amethyst::ecs::resources::{Camera, Projection, Time};
use amethyst::ecs::systems::TransformSystem;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::{Pipeline, VertexPosNormal};
use amethyst::config::Config;
use amethyst::WindowEvent;

use input::{InputContext, Action};

mod input;
mod input_mapper;

const PLANK_VELOCITY : f32 = 2.0;
const BALL_VELOCITY : f32 = 0.6;
const BALL_BOUNCE_VELOCITY_INCREASE : f32 = 1.2;

struct Ball {
    pub position : cgmath::Point2<f32>,
    pub velocity : cgmath::Vector2<f32>,
    pub radius : f32
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: cgmath::Point2::new(0.0, 0.0),
            velocity: cgmath::Vector2::new(0.0, 0.0),
            radius: 0.02
        }
    }

    pub fn start(&mut self) {
        let x = if rand::random::<bool>() { 1. } else { -1. };
        let y = rand::thread_rng().gen_range::<f32>(-1., 1.);
        let v = BALL_VELOCITY;
        self.velocity = cgmath::Vector2::new(x, y).normalize() * v;
    }

    pub fn transform(&self, t : &mut LocalTransform) {
        t.translation = [self.position.x, self.position.y, 0.0];
        t.scale = [self.radius, self.radius, 1.0];
    }

    pub fn update_position(&mut self,
                           dt : f32,
                           left : f32,
                           right : f32,
                           top : f32,
                           bottom : f32,
                           collision_data : &Vec<PlankCollisionData>) -> Option<PlankMiss> {
        let start_position = self.position.clone();
        self.position += self.velocity * dt;
        let end_position = self.position;

        for cd in collision_data {
            match cd.collision_test(&start_position, &end_position) {
                Some(result) => {
                    self.velocity.x = -self.velocity.x;
                    self.velocity *= BALL_BOUNCE_VELOCITY_INCREASE;
                    self.position.x = result.new_x;
                },
                None => ()
            };
        }

        if self.position.y + self.radius >= top {
            self.velocity.y = -self.velocity.y;
            self.position.y -= (self.position.y + self.radius - top) * 2.;
        }
        if self.position.y - self.radius <= bottom {
            self.velocity.y = -self.velocity.y;
            self.position.y -= (self.position.y - self.radius - bottom) * 2.;
        }
        if self.position.x < left {
            self.position = cgmath::Point2::new(0., 0.);
            self.velocity = cgmath::Vector2::new(0., 0.);
            return Some(PlankMiss { side : Side::Left });
        }
        if self.position.x > right {
            self.position = cgmath::Point2::new(0., 0.);
            self.velocity = cgmath::Vector2::new(0., 0.);
            return Some(PlankMiss { side : Side::Right });
        }

        None
    }
}

pub struct PlankMiss {
    side : Side
}

impl Component for Ball {
    type Storage = VecStorage<Ball>;
}

#[derive(PartialEq, Clone, Debug)]
pub enum Side {
    Left,
    Right
}

enum Direction {
    Up,
    Down
}

struct Plank {
    pub position : f32,
    pub velocity_up : f32,
    pub velocity_down : f32,
    pub dimensions : cgmath::Vector2<f32>,
    pub side: Side
}

#[derive(Debug)]
pub struct PlankCollisionData {
    side : Side,
    x : f32,
    y_top : f32,
    y_bottom : f32
}

impl PlankCollisionData {

    pub fn new(side : Side, x : f32, y_top : f32, y_bottom : f32) -> PlankCollisionData {
        PlankCollisionData {
            side : side,
            x : x,
            y_bottom : y_bottom,
            y_top : y_top
        }
    }

    pub fn collision_test(&self,
                          start : &cgmath::Point2<f32>,
                          end : &cgmath::Point2<f32>) -> Option<PlankCollisionResult> {
        match self.side {
            Side::Left => {
                if start.x >= self.x && end.x < self.x {
                    let ray = collision::Ray2::new(start.clone(), (end - start).normalize());
                    let line = collision::Line2::new(cgmath::Point2::<f32>::new(self.x, self.y_bottom),
                                                     cgmath::Point2::<f32>::new(self.x, self.y_top));
                    match (ray, line).intersection() {
                        Some(_) =>
                            Some(PlankCollisionResult::new(self.x - (end.x - self.x))),
                        None => None
                    }
                } else {
                    None
                }
            },
            Side::Right => {
                if start.x <= self.x && end.x > self.x {
                    let ray = collision::Ray2::new(start.clone(), (end - start).normalize());
                    let line = collision::Line2::new(cgmath::Point2::<f32>::new(self.x, self.y_bottom),
                                                     cgmath::Point2::<f32>::new(self.x, self.y_top));
                    match (ray, line).intersection() {
                        Some(_) =>
                            Some(PlankCollisionResult::new(self.x - (end.x - self.x))),
                        None => None
                    }
                } else {
                    None
                }
            }
        }
    }

}

pub struct PlankCollisionResult {
    new_x : f32
}

impl PlankCollisionResult {
    pub fn new(new_x: f32) -> PlankCollisionResult {
        PlankCollisionResult {
            new_x : new_x
        }
    }
}
impl Plank {
    pub fn new(side : Side) -> Plank {
        Plank {
            position : 0.0,
            velocity_down : 0.,
            velocity_up : 0.,
            dimensions : cgmath::Vector2::new(0.01, 0.3),
            side : side
        }
    }

    pub fn transform(&self, t : &mut LocalTransform, left : f32, right : f32) {
        t.scale = [self.dimensions.x, self.dimensions.y, 1.0];
        match self.side {
            Side::Left => t.translation = [left + self.dimensions.x/2., self.position, 0.0],
            Side::Right => t.translation = [right - self.dimensions.x/2., self.position, 0.0],
        };
    }

    pub fn update_position(&mut self, dt : f32, top_bound : f32, bottom_bound: f32) {
        self.position += self.velocity_up * dt;
        self.position -= self.velocity_down * dt;

        if (self.position + self.dimensions.y/2.) >= top_bound {
            self.position = top_bound - self.dimensions.y/2.;
            self.velocity_down = 0.;
            self.velocity_up = 0.;
        }

        if (self.position - self.dimensions.y/2.) <= bottom_bound {
            self.position = bottom_bound + self.dimensions.y/2.;
            self.velocity_up = 0.;
            self.velocity_down = 0.;
        }
    }

    pub fn collision_data(&self, left : f32, right : f32) -> PlankCollisionData {
        let x = match self.side {
            Side::Left => left + self.dimensions.x,
            Side::Right => right - self.dimensions.x
        };
        PlankCollisionData::new(self.side.clone(),
                                x,
                                self.position + self.dimensions.y/2.,
                                self.position - self.dimensions.y/2.)
    }
}

impl Component for Plank {
    type Storage = VecStorage<Plank>;
}

struct GameState {
    pub left_score : u32,
    pub right_score : u32,
    pub round_active : bool,
    pub round : u32
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            left_score : 0,
            right_score: 0,
            round_active : false,
            round : 1
        }
    }
}

struct PongSystem {
    reader_id : Option<shrev::ReaderId>
}

impl PongSystem {
    pub fn new() -> PongSystem {
        PongSystem {
            reader_id : None
        }
    }
}

impl<'a> System<'a> for PongSystem {
    type SystemData = (WriteStorage<'a, Ball>,
                       WriteStorage<'a, Plank>,
                       WriteStorage<'a, LocalTransform>,
                       Fetch<'a, Camera>,
                       Fetch<'a, Time>,
                       FetchMut<'a, GameState>,
                       FetchMut<'a, shrev::EventHandler>);

    #[allow(unused_variables)]
    #[allow(unused_mut)]
    fn run(&mut self,
           (mut balls, mut planks, mut locals, camera, time, mut game_state, mut input): Self::SystemData) {
        let reader_id = match self.reader_id {
            Some(reader_id) => reader_id,
            None => input.register_reader::<ControllerEvent>()
        };
        self.reader_id = Some(reader_id);

        // process plank controller input
        let mut start_ball = false;
        for event in input.read::<ControllerEvent>(reader_id) {
            match event.payload {
                remawin::ControllerEvent::State(action, state, _, _) => {
                    if state == remawin::event::StateAction::Activated
                        || state == remawin::event::StateAction::Deactivated {
                        match action {
                            Action::LeftPaddleDown => update_velocity(&mut planks, Side::Left, Direction::Down, state),
                            Action::LeftPaddleUp => update_velocity(&mut planks, Side::Left, Direction::Up, state),
                            Action::RightPaddleDown => update_velocity(&mut planks, Side::Right, Direction::Down, state),
                            Action::RightPaddleUp => update_velocity(&mut planks, Side::Right, Direction::Up, state),
                            _ => ()
                        };
                    }
                }
                remawin::ControllerEvent::Action(Action::StartRound, _) => {
                    if !game_state.round_active {
                        game_state.round_active = true;
                        start_ball = true;
                    };
                },
                _ => ()
            };
        }

        let (left_bound, right_bound, top_bound, bottom_bound) = match camera.proj {
            Projection::Orthographic {
                left,
                right,
                top,
                bottom,
                ..
            } => (left, right, top, bottom),
            _ => (1.0, 1.0, 1.0, 1.0),
        };

        let delta_time = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        let mut plank_collision_data : Vec<PlankCollisionData> = Vec::default();
        // update plank positions
        // do plank/boundary collision testing
        for (plank, mut local) in (&mut planks, &mut locals).join() {
            plank.update_position(delta_time, top_bound, bottom_bound);
            plank.transform(&mut local, left_bound, right_bound);
            plank_collision_data.push(plank.collision_data(left_bound, right_bound));
        }

        let mut miss : Option<PlankMiss> = None;
        // update ball position
        // do ball/plank collision testing
        for (ball, mut local) in (&mut balls, &mut locals).join() {
            if start_ball {
                ball.start();
            }
            match ball.update_position(delta_time,
                                       left_bound,
                                       right_bound,
                                       top_bound,
                                       bottom_bound,
                                       &plank_collision_data) {
                Some(m) => miss = Some(m),
                None => ()
            };
            ball.transform(&mut local);
        }

        // if applicable, do scoring and start new round
        match miss {
            Some(miss) => {
                game_state.round += 1;
                game_state.round_active = false;
                match miss.side {
                    Side::Left => game_state.right_score += 1,
                    Side::Right => game_state.left_score += 1,
                }
                println!("{:?} player missed the ball! Score is {} - {}",
                         miss.side,
                         game_state.left_score,
                         game_state.right_score);
            },
            None => ()
        };
    }
}

fn update_velocity(planks : &mut WriteStorage<Plank>,
                   side : Side,
                   direction : Direction,
                   state : remawin::event::StateAction) {
    let new_velocity = match state {
        remawin::event::StateAction::Activated | remawin::event::StateAction::Active => PLANK_VELOCITY,
        remawin::event::StateAction::Deactivated => 0.0,
    };
    for plank in (planks).join() {
        if plank.side == side {
            match direction {
                Direction::Up => plank.velocity_up = new_velocity,
                Direction::Down => plank.velocity_down = new_velocity
            }
        }
    }
}

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
        let mut transform = LocalTransform::default();
        ball.transform(&mut transform);
        world
            .create_entity()
            .with(square.clone())
            .with(ball)
            .with(transform)
            .with(Transform::default())
            .build();

        let plank = Plank::new(Side::Left);
        let mut transform = LocalTransform::default();
        plank.transform(&mut transform, left_bound, right_bound);
        world
            .create_entity()
            .with(square.clone())
            .with(plank)
            .with(transform)
            .with(Transform::default())
            .build();

        // Create right plank entity
        let plank = Plank::new(Side::Right);
        let mut transform = LocalTransform::default();
        plank.transform(&mut transform, left_bound, right_bound);
        world
            .create_entity()
            .with(square.clone())
            .with(plank)
            .with(transform)
            .with(Transform::default())
            .build();

    }

    fn handle_events(&mut self,
                     events: &[WindowEvent],
                     world: &mut World,
                     _: &mut AssetManager,
                     _: &mut Pipeline)
                     -> Trans {
        let mut input = world.write_resource::<input_mapper::AmethystEventMapper<Action, InputContext>>();
        let mut event_handler = world.write_resource::<shrev::EventHandler>();
        for me in input.process(&events.to_vec()) {
            match me {
                remawin::Event::Controller(remawin::ControllerEvent::Action(Action::Exit, _)) |
                remawin::Event::Window(remawin::WindowEvent::Close) => {
                    return Trans::Quit;
                },
                remawin::Event::Window(event) => {
                    event_handler.write_single(&IWindowEvent::new(event));
                },
                remawin::Event::Controller(event) => {
                    event_handler.write_single(&ControllerEvent::new(event));
                }
            }
        }
        Trans::None
    }
}

#[derive(Clone, Debug)]
pub struct IWindowEvent {
    pub payload : remawin::WindowEvent
}

impl IWindowEvent {
    pub fn new(event: remawin::WindowEvent) -> IWindowEvent {
        IWindowEvent {
            payload : event
        }
    }
}

#[derive(Clone, Debug)]
pub struct ControllerEvent {
    pub payload : remawin::ControllerEvent<Action, InputContext>
}

impl ControllerEvent {
    pub fn new(event: remawin::ControllerEvent<Action, InputContext>) -> ControllerEvent {
        ControllerEvent {
            payload : event
        }
    }
}

impl shrev::Event for ControllerEvent {}
impl shrev::Event for IWindowEvent {}

fn main() {
    let path = format!("{}/02_pong/resources/config.yml",
                       env!("CARGO_MANIFEST_DIR"));
    let input_path = format!("{}/02_pong/resources/bindings.yml",
                       env!("CARGO_MANIFEST_DIR"));
    let cfg = DisplayConfig::load(path);
    let dim = cfg.dimensions.unwrap().clone();
    let mut game = Application::build(Pong, cfg)
        .register::<Ball>()
        .register::<Plank>()
        .with::<PongSystem>(PongSystem::new(), "pong_system", &[])
        .with::<TransformSystem>(TransformSystem::new(), "transform_system", &["pong_system"])
        .done();
    let mut event_handler = shrev::EventHandler::new();
    event_handler.register::<IWindowEvent>();
    event_handler.register::<ControllerEvent>();
    let mut event_mapper = input_mapper::AmethystEventMapper::<Action, InputContext>::new(
        (dim.0 as f64, dim.1 as f64));
    event_mapper.remapper_mut()
        .with_bindings_file(&input_path)
        .activate_context(&InputContext::Default, 1);
    game.world_mut().add_resource(event_mapper);
    game.world_mut().add_resource(event_handler);
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