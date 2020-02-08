use crate::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
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
