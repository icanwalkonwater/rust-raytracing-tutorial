use crate::utils::random_in_unit_sphere;
use crate::vec3::{cross, VEC3_UP};
use crate::{Ray, Vec3, RESOLUTION};
use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    bottom_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let look_from = Vec3(8.0, 2.0, 3.0);
        let look_at = Vec3(0.0, 0.0, 0.0);
        let dist_to_focus = (look_from - look_at).length();
        let aperture = 0.5;

        Camera::new(
            look_from,
            look_at,
            VEC3_UP,
            35.0,
            RESOLUTION.0 as f32 / RESOLUTION.1 as f32,
            aperture,
            dist_to_focus,
        )
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        fov_vertical: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let origin = look_from;

        let theta = fov_vertical * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalize();
        let u = cross(view_up, w).normalize();
        let v = cross(w, u);

        Camera {
            bottom_left_corner: origin
                - u * half_width * focus_dist
                - v * half_height * focus_dist
                - w * focus_dist,
            horizontal: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            lens_radius: aperture / 2.0,
            origin,
            u,
            v,
            _w: w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let random = random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * random.x() + self.v * random.y();

        Ray::new(
            self.origin + offset,
            self.bottom_left_corner + self.horizontal * u + self.vertical * v
                - self.origin
                - offset,
        )
    }
}
