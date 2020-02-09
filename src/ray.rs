use crate::Vec3;

#[derive(Debug)]
pub struct Ray<'a> {
    origin: &'a Vec3,
    direction: Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        let mut interpolation = &self.direction * t;
        interpolation += &self.origin;
        interpolation
    }
}
