use crate::{Vec3, Ray};

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, interval: (f32, f32)) -> Option<HitRecord>;
}
