pub use crate::camera::Camera;
pub use crate::objects::HitRecord;
pub use crate::objects::Hitable;
pub use crate::objects::HitableVec;
pub use crate::objects::Sphere;
pub use crate::ray::Ray;
pub use crate::tracing::color_ray;
pub use crate::vec3::dot;
pub use crate::vec3::Color;
pub use crate::vec3::Vec3;
pub use crate::vec3::VEC3_UNIT;
pub use crate::vec3::VEC3_ZERO;

mod camera;
mod objects;
mod ray;
mod tracing;
mod vec3;
