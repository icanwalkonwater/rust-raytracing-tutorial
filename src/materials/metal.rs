use crate::utils::{random_in_unit_sphere, reflect};
use crate::{dot, Color, HitRecord, Material, Ray, Vec3};

pub struct Metal {
    albedo: Color,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Color, mut fuzziness: f32) -> Self {
        if fuzziness > 1.0 {
            fuzziness = 1.0;
        }

        Metal { albedo, fuzziness }
    }

    pub fn new_boxed(albedo: Color, fuzziness: f32) -> Box<Self> {
        Box::new(Self::new(albedo, fuzziness))
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray_in.direction().normalize(), record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + random_in_unit_sphere() * self.fuzziness,
        );

        if dot(scattered.direction(), record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
