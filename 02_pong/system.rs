use shrev;
use cgmath;
use remawin;
use rand;

use cgmath::InnerSpace;

use rand::Rng;

use amethyst::ecs::{Fetch, FetchMut, Join, System, WriteStorage};
use amethyst::ecs::components::LocalTransform;
use amethyst::ecs::resources::{Camera, Projection, Time};

use input::*;
use comp::*;
use coll::*;
use event::*;

const PLANK_VELOCITY : f32 = 2.0;
const BALL_VELOCITY : f32 = 0.6;
const BALL_BOUNCE_VELOCITY_INCREASE : f32 = 1.2;

pub struct GameState {
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

pub struct PongSystem {
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
           (mut balls, mut planks, mut locals, camera, time, mut game_state, mut events): Self::SystemData) {
        let mut reader_id = match self.reader_id {
            Some(reader_id) => reader_id,
            None => events.register_reader::<ControllerEvent>()
        };

        // process plank controller input
        let mut start_ball = false;
        for event in events.read::<ControllerEvent>(&mut reader_id) {
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
        self.reader_id = Some(reader_id);

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

        let dt = time.delta_time.subsec_nanos() as f32 / 1.0e9;

        let mut plank_collision_data : Vec<PlankCollisionData> = Vec::default();
        // update plank positions
        // do plank/boundary collision testing
        for (plank, mut local) in (&mut planks, &mut locals).join() {

            // update plank position
            plank.position += plank.velocity_up * dt;
            plank.position -= plank.velocity_down * dt;

            // do boundary collision testing and response
            if (plank.position + plank.dimensions.y/2.) >= top_bound {
                plank.position = top_bound - plank.dimensions.y/2.;
                plank.velocity_down = 0.;
                plank.velocity_up = 0.;
            }

            if (plank.position - plank.dimensions.y/2.) <= bottom_bound {
                plank.position = bottom_bound + plank.dimensions.y/2.;
                plank.velocity_up = 0.;
                plank.velocity_down = 0.;
            }

            // update transform
            local.scale = [plank.dimensions.x, plank.dimensions.y, 1.0];
            match plank.side {
                Side::Left => local.translation = [left_bound + plank.dimensions.x/2., plank.position, 0.0],
                Side::Right => local.translation = [right_bound - plank.dimensions.x/2., plank.position, 0.0],
            };

            // store plank collision data for ball collision testing
            let x = match plank.side {
                Side::Left => left_bound + plank.dimensions.x,
                Side::Right => right_bound - plank.dimensions.x
            };
            plank_collision_data.push(PlankCollisionData::new(plank.side.clone(),
                                                              x,
                                                              plank.position + plank.dimensions.y/2.,
                                                              plank.position - plank.dimensions.y/2.));
        }

        // update ball position
        // do ball/plank collision testing
        for (ball, mut local) in (&mut balls, &mut locals).join() {

            // if round started, we randomize a velocity for the ball
            if start_ball {
                let x = if rand::random::<bool>() { 1. } else { -1. };
                let y = rand::thread_rng().gen_range::<f32>(-1., 1.);
                let v = BALL_VELOCITY;
                ball.velocity = cgmath::Vector2::new(x, y).normalize() * v;
            }

            // update position of ball
            let start_position = ball.position.clone();
            ball.position += ball.velocity * dt;
            let end_position = ball.position;

            // check for plank collisions, and calculate response
            for cd in &plank_collision_data {
                match cd.collision_test(&start_position, &end_position) {
                    Some(result) => {
                        ball.velocity.x = -ball.velocity.x;
                        ball.velocity *= BALL_BOUNCE_VELOCITY_INCREASE;
                        ball.position.x = result.new_x;
                    },
                    None => ()
                };
            }

            // check for boundary collision at top/bottom (should bounce)
            if ball.position.y + ball.radius >= top_bound {
                ball.velocity.y = -ball.velocity.y;
                ball.position.y -= (ball.position.y + ball.radius - top_bound) * 2.;
            }
            if ball.position.y - ball.radius <= bottom_bound {
                ball.velocity.y = -ball.velocity.y;
                ball.position.y -= (ball.position.y - ball.radius - bottom_bound) * 2.;
            }

            // check for boundary collision at left/right (ends round and assigns points to victor)
            if ball.position.x < left_bound {
                ball.position = cgmath::Point2::new(0., 0.);
                ball.velocity = cgmath::Vector2::new(0., 0.);

                game_state.round += 1;
                game_state.round_active = false;
                game_state.right_score += 1;
                println!("Left player missed the ball! Score is {} - {}",
                         game_state.left_score,
                         game_state.right_score);
            }
            if ball.position.x > right_bound {
                ball.position = cgmath::Point2::new(0., 0.);
                ball.velocity = cgmath::Vector2::new(0., 0.);

                game_state.round += 1;
                game_state.round_active = false;
                game_state.left_score += 1;
                println!("Right player missed the ball! Score is {} - {}",
                         game_state.left_score,
                         game_state.right_score);
            }

            // update transform
            local.translation = [ball.position.x, ball.position.y, 0.0];
            local.scale = [ball.radius, ball.radius, 1.0];
        }

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