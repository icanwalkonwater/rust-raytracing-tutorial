use crate::objects::hitable::{HitRecord, Hitable};
use crate::{dot, Material, Ray, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, (min, max): (f32, f32)) -> Option<HitRecord> {
        let origin_to_center = ray.origin() - self.center;

        let a = dot(ray.direction(), ray.direction());
        let b = dot(origin_to_center, ray.direction());
        let c = dot(origin_to_center, origin_to_center) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            // No solutions, the ray didn't hit the sphere
            None
        } else {
            // Compute first root, which is where along the ray we have hit the sphere
            let temp = (-b - discriminant.sqrt()) / a;

            if temp < max && temp > min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;

                Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    material: &*self.material,
                })
            } else {
                // Compute second root
                let temp = (-b + discriminant.sqrt()) / a;
                if temp < max && temp > min {
                    let point = ray.point_at(temp);
                    let normal = (point - self.center) / self.radius;

                    Some(HitRecord {
                        t: temp,
                        point,
                        normal,
                        material: &*self.material,
                    })
                } else {
                    None
                }
            }
        }
    }
}
