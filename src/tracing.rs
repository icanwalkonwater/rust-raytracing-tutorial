use crate::{Color, dot, Ray, Vec3};

static SKYBOX_TOP_COLOR: Color = Vec3(0.5, 0.7, 1.0);
static SKYBOX_BOTTOM_COLOR: Color = Vec3(1.0, 1.0, 1.0);

/// Entry point of the ray.
pub fn color(ray: &Ray) -> Color {
    if hit_sphere(&Vec3::new(-0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let ray_normalized = ray.direction().normalize();
    // Get where is the ray pointing to between 0 and 1
    let percent_vertical = 0.5 * (ray_normalized.y() + 1.0);

    // Lerp between two colors to make a gradient
    &(&SKYBOX_BOTTOM_COLOR * (1.0 - percent_vertical)) + &(&SKYBOX_TOP_COLOR * percent_vertical)
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;

    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}
