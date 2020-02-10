use crate::utils::random_in_unit_sphere;
use crate::{Color, HitRecord, Material, Ray, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }

    pub fn new_boxed(albedo: Color) -> Box<Self> {
        Box::new(Self::new(albedo))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = record.point + record.normal + random_in_unit_sphere();
        Some((Ray::new(record.point, target - record.point), self.albedo))
    }
}
