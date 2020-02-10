use crate::{Color, Hitable, Ray, Vec3};
use rand::distributions::{Distribution, Uniform};

static SKYBOX_TOP_COLOR: Color = Vec3(0.5, 0.7, 1.0);
static SKYBOX_BOTTOM_COLOR: Color = Vec3(1.0, 1.0, 1.0);

/// Entry point of the ray.
pub fn color_ray(ray: &Ray, world: &dyn Hitable) -> Color {
    if let Some(record) = world.hit(ray, (0.001, std::f32::MAX)) {
        // Bounce on the surface with a random angle
        let target = record.point + record.normal + random_in_unit_sphere();
        color_ray(&Ray::new(record.point, target - record.point), world) * 0.5

        // Draw normals
        // (record.normal + VEC3_UNIT) * 0.5
    } else {
        let ray_normalized = ray.direction().normalize();
        // Get where is the ray pointing to between 0 and 1
        let percent_vertical = 0.5 * (ray_normalized.y() + 1.0);
        // Lerp between two colors to make a gradient
        SKYBOX_BOTTOM_COLOR * (1.0 - percent_vertical) + SKYBOX_TOP_COLOR * percent_vertical
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut random = rand::thread_rng();
    let uniform_sampler = Uniform::from(0.0f32..1.0f32);

    loop {
        let point = Vec3(
            uniform_sampler.sample(&mut random),
            uniform_sampler.sample(&mut random),
            uniform_sampler.sample(&mut random),
        );

        if point.length_squared() < 1.0 {
            return point;
        }
    }
}

/*
fn _hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> Option<f32> {
    let origin_to_center = ray.origin() - center;

    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(origin_to_center, ray.direction());
    let c = dot(origin_to_center, origin_to_center) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No solutions, the ray didn't hit the sphere
        None
    } else {
        // Compute the first root, where the ray has hit the sphere first
        Some((-b - discriminant.sqrt()) / (a * 2.0))
    }
}
*/