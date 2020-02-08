use rust_raytracing_tutorial::{color, Ray, Vec3, VEC3_ZERO};

fn main() {
    let width = 200;
    let height = 100;

    // Print header
    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let viewport_bottom_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let viewport_horizontal = Vec3::new(4.0, 0.0, 0.0);
    let viewport_vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = VEC3_ZERO.clone();

    for y in (0..height).rev() {
        for x in 0..width {
            // Screen space coordinates
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let ray = Ray::new(
                origin.clone(),
                &(&viewport_bottom_left_corner + &(&viewport_horizontal * u)) + &(&viewport_vertical * v),
            );
            let mut color = color(&ray);

            color *= 255.0;

            println!("{:.0} {:.0} {:.0}", color.r(), color.g(), color.b());
        }
    }
}
