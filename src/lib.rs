pub use crate::camera::Camera;
pub use crate::materials::Dielectric;
pub use crate::materials::Lambertian;
pub use crate::materials::Material;
pub use crate::materials::Metal;
pub use crate::objects::HitRecord;
pub use crate::objects::Hitable;
pub use crate::objects::HitableVec;
pub use crate::objects::Sphere;
pub use crate::ray::Ray;
pub use crate::tracing::color_ray;
pub use crate::vec3::dot;
pub use crate::vec3::Color;
pub use crate::vec3::Vec3;
pub use crate::vec3::VEC3_BACKWARD;
pub use crate::vec3::VEC3_DOWN;
pub use crate::vec3::VEC3_FORWARD;
pub use crate::vec3::VEC3_LEFT;
pub use crate::vec3::VEC3_RIGHT;
pub use crate::vec3::VEC3_UNIT;
pub use crate::vec3::VEC3_UP;
pub use crate::vec3::VEC3_ZERO;

mod camera;
mod materials;
mod objects;
mod ray;
mod tracing;
mod utils;
mod vec3;

// pub const RESOLUTION: (u32, u32) = (200, 100);
pub const RESOLUTION: (u32, u32) = (1280, 720);
pub const AA_SAMPLES: u32 = 100; // default: 100
