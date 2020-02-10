use crate::{Color, Hitable, Ray, Vec3};

static SKYBOX_TOP_COLOR: Color = Vec3(0.5, 0.7, 1.0);
static SKYBOX_BOTTOM_COLOR: Color = Vec3(1.0, 1.0, 1.0);

/// Entry point of the ray.
pub fn color_ray(ray: &Ray, world: &dyn Hitable, depth: u32) -> Color {
    if let Some(record) = world.hit(ray, (0.001, std::f32::MAX)) {
        match record.material.scatter(ray, &record) {
            Some((scattered, attenuation)) if depth < 50 => {
                attenuation * color_ray(&scattered, world, depth + 1)
            }
            _ => Color::default(),
        }

    // Bounce on the surface with a random angle
    // let target = record.point + record.normal + random_in_unit_sphere();
    // color_ray(&Ray::new(record.point, target - record.point), world) * 0.5

    // Draw normals
    // (record.normal + VEC3_UNIT) * 0.5
    } else {
        let ray_normalized = ray.direction().normalize();
        // Get where is the ray pointing to between 0 and 1
        let percent_vertical = 0.5 * (ray_normalized.y() + 1.0);
        // Lerp between two colors to make a gradient
        SKYBOX_BOTTOM_COLOR * (1.0 - percent_vertical) + SKYBOX_TOP_COLOR * percent_vertical
    }
}
