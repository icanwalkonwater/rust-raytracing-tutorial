use crate::objects::hitable::{HitRecord, Hitable};
use crate::{Material, Ray, Sphere, Vec3};

pub struct HitableVec {
    vec: Vec<Box<dyn Hitable>>,
}

impl HitableVec {
    pub fn new() -> Self {
        HitableVec { vec: Vec::new() }
    }

    pub fn vec(&mut self) -> &mut Vec<Box<dyn Hitable>> {
        &mut self.vec
    }

    pub fn push_sphere(&mut self, center: Vec3, radius: f32, material: Box<dyn Material>) {
        self.vec
            .push(Box::new(Sphere::new(center, radius, material)));
    }
}

impl From<Vec<Box<dyn Hitable>>> for HitableVec {
    fn from(vec: Vec<Box<dyn Hitable>>) -> Self {
        HitableVec { vec }
    }
}

impl Hitable for HitableVec {
    fn hit(&self, ray: &Ray, (min, max): (f32, f32)) -> Option<HitRecord> {
        let mut result_record = None;
        let mut closest = max;

        for object in self.vec.iter() {
            if let Some(record) = object.hit(ray, (min, closest)) {
                closest = record.t;
                result_record = Some(record);
            }
        }

        result_record
    }
}
