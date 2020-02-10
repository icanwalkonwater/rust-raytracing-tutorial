use image::{ImageBuffer, Rgb};
use rand::distributions::{Distribution, Uniform};
use rust_raytracing_tutorial::{color_ray, Camera, Color, HitableVec, Sphere, Vec3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buffer = create_image();
    buffer.save("test.png")?;

    Ok(())
}

fn create_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = 200;
    let height = 100;
    let sample_amount = 100;

    let mut buffer = ImageBuffer::new(width, height);

    let mut world = HitableVec::new();
    world
        .vec()
        .push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world
        .vec()
        .push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::default();

    let mut random = rand::thread_rng();
    let uniform_sampler = Uniform::from(0.0f32..1.0f32);

    // For each pixel in the viewport
    for y in 0..height {
        for x in 0..width {
            let mut color: Color = Color::default();

            // Sample
            for _ in 0..sample_amount {
                // Screen space coordinates
                let u = (x as f32 + uniform_sampler.sample(&mut random)) / width as f32;
                let v = (y as f32 + uniform_sampler.sample(&mut random)) / height as f32;

                let ray = camera.get_ray(u, v);
                let _point = ray.point_at(2.0);

                color += color_ray(&ray, &world);
            }

            color /= sample_amount as f32;
            // Gamma correction
            color = Vec3(color.0.sqrt(), color.1.sqrt(), color.2.sqrt());

            // Put it back in range 0..256
            color *= 255.0;
            buffer.put_pixel(
                x,
                height - y - 1, // y is descending in the image
                Rgb([color.r() as u8, color.g() as u8, color.b() as u8]),
            );
        }

        if y % 4 == 0 {
            println!("Progress: {:0.0}%", y as f32 / height as f32 * 100.0);
        }
    }

    buffer
}
