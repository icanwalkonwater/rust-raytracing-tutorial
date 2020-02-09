pub use crate::objects::Hitable;
pub use crate::objects::HitableVec;
pub use crate::objects::HitRecord;
pub use crate::objects::Sphere;
pub use crate::ray::Ray;
pub use crate::tracing::color;
pub use crate::vec3::Color;
pub use crate::vec3::dot;
pub use crate::vec3::Vec3;
pub use crate::vec3::VEC3_UNIT;
pub use crate::vec3::VEC3_ZERO;

mod ray;
mod vec3;
mod tracing;
mod objects;
