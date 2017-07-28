use cgmath;
use collision;

use super::comp::Side;

use collision::Intersect;
use cgmath::InnerSpace;

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
    pub new_x : f32
}

impl PlankCollisionResult {
    pub fn new(new_x: f32) -> PlankCollisionResult {
        PlankCollisionResult {
            new_x : new_x
        }
    }
}
