use crate::{Color, HitRecord, Ray};

pub trait Material {
    /// If the incident ray isn't absorbed, it need to produce a scattered ray and its attenuation.
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}
