use crate::{Material, Ray, Vec3};

pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, interval: (f32, f32)) -> Option<HitRecord>;
}
