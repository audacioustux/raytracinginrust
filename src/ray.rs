use crate::vec3::{Position, Vec3f};

#[derive(Clone, Copy)]
pub struct Ray {
    pub a: Vec3f<Position>,
    pub b: Vec3f<Position>,
}

impl Ray {
    pub fn origin(&self) -> Vec3f<Position> {
        self.a
    }
    pub fn direction(&self) -> Vec3f<Position> {
        self.b
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3f<Position> {
        self.a + t * self.b
    }
}
