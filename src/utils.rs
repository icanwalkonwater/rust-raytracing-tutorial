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

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
