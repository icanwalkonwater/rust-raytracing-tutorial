use crate::utils::{reflect, refract, schlick};
use crate::{dot, HitRecord, Material, Ray, Vec3, VEC3_UNIT};

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }

    pub fn new_boxed(ref_idx: f32) -> Box<Self> {
        Box::new(Self::new(ref_idx))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let (outward_normal, ni_over_nt, cosine) = if dot(ray_in.direction(), record.normal) > 0.0 {
            (
                -record.normal,
                self.ref_idx,
                self.ref_idx * dot(ray_in.direction(), record.normal) / ray_in.direction().length(),
            )
        } else {
            (
                record.normal,
                1.0 / self.ref_idx,
                -dot(ray_in.direction(), record.normal) / ray_in.direction().length(),
            )
        };

        let (refracted, reflect_prob) =
            if let Some(refracted) = refract(ray_in.direction(), outward_normal, ni_over_nt) {
                (Some(refracted), schlick(cosine, self.ref_idx))
            } else {
                (None, 1.0)
            };

        if rand::random::<f32>() < reflect_prob {
            let reflected = reflect(ray_in.direction(), record.normal);
            Some((Ray::new(record.point, reflected), VEC3_UNIT))
        } else {
            Some((Ray::new(record.point, refracted.unwrap()), VEC3_UNIT))
        }
    }
}
