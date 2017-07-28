use cgmath;
use amethyst::ecs::{VecStorage, Component};

pub struct Ball {
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
}

impl Component for Ball {
    type Storage = VecStorage<Ball>;
}

#[derive(PartialEq, Clone, Debug)]
pub enum Side {
    Left,
    Right
}

pub enum Direction {
    Up,
    Down
}

pub struct Plank {
    pub position : f32,
    pub velocity_up : f32,
    pub velocity_down : f32,
    pub dimensions : cgmath::Vector2<f32>,
    pub side: Side
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
}

impl Component for Plank {
    type Storage = VecStorage<Plank>;
}
