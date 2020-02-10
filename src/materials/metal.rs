use crate::utils::reflect;
use crate::{dot, Color, HitRecord, Material, Ray, Vec3};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray_in.direction().normalize(), record.normal);

        let scattered = Ray::new(record.point, reflected);

        if dot(scattered.direction(), record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
