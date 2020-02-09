use crate::{dot, Ray, Vec3};
use crate::objects::hitable::{Hitable, HitRecord};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, (min, max): (f32, f32)) -> Option<HitRecord> {
        let origin_to_center = ray.origin() - &self.center;

        let a = dot(ray.direction(), ray.direction());
        let b = dot(&origin_to_center, ray.direction());
        let c = dot(&origin_to_center, &origin_to_center) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            // No solutions, the ray didn't hit the sphere
            None
        } else {
            // Compute first root, which is where along the ray we have hit the sphere
            let temp = (-b - discriminant.sqrt()) / a;

            if temp < max && temp > min {
                let point = ray.point_at(temp);
                let normal = (&point - &self.center) / self.radius;

                Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                })
            } else {
                // Compute second root
                let temp = (-b + discriminant.sqrt()) / a;
                if temp < max && temp > min {
                    let point = ray.point_at(temp);
                    let normal = (&point - &self.center) / self.radius;

                    Some(HitRecord {
                        t: temp,
                        point,
                        normal,
                    })
                } else {
                    None
                }
            }
        }
    }
}