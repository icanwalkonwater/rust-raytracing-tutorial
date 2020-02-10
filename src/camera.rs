use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    bottom_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3(0.0, 0.0, 0.0),
            bottom_left_corner: Vec3(-2.0, -1.0, -1.0),
            horizontal: Vec3(4.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    pub fn new(origin: Vec3, bottom_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Self {
        Camera {
            origin,
            bottom_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
