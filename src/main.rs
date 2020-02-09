use std::fs::File;
use std::io::Write;

use rust_raytracing_tutorial::{color, HitableVec, Ray, Sphere, Vec3, VEC3_ZERO};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("./test.ppm")?;
    file.write_all(create_image()?.as_slice())?;

    Ok(())
}

fn create_image() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    let width = 200;
    let height = 100;

    // Print header
    writeln!(buffer, "P3\n{} {}\n255", width, height)?;

    let viewport_bottom_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let viewport_horizontal = Vec3::new(4.0, 0.0, 0.0);
    let viewport_vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = VEC3_ZERO.clone();

    let mut objects = HitableVec::new();
    objects.vec().push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    objects.vec().push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for y in (0..height).rev() {
        for x in 0..width {
            // Screen space coordinates
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(
                &origin,
                &viewport_bottom_left_corner + &(&viewport_horizontal * u) + &viewport_vertical * v,
            );
            let mut color = color(&ray, &objects);

            color *= 255.0;

            writeln!(buffer, "{:.0} {:.0} {:.0}", color.r(), color.g(), color.b())?;
        }
    }

    Ok(buffer)
}