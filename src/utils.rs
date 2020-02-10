use rand::distributions::{Distribution, Uniform};

use crate::{dot, Vec3};

pub fn random_in_unit_sphere() -> Vec3 {
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

pub fn reflect(vector: Vec3, normal: Vec3) -> Vec3 {
    vector - normal * 2.0 * dot(vector, normal)
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
