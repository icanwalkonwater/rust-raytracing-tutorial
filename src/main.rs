use image::{ImageBuffer, Rgb};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use rust_raytracing_tutorial::{
    color_ray, Camera, Color, Dielectric, HitableVec, Lambertian, Material, Metal, Sphere, Vec3,
    AA_SAMPLES, RESOLUTION,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buffer = create_image();
    buffer.save("test.png")?;

    Ok(())
}

fn create_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    // Image dimensions
    let (width, height) = RESOLUTION;
    // Amount of sample to use for the anti aliasing
    let sample_amount = AA_SAMPLES;

    // The pixels will be written to this buffer
    let mut buffer = ImageBuffer::new(width, height);

    // Main camera
    let camera = Camera::default();

    // Gather the objects of the world
    let world = create_random_scene();

    // Initialize an uniform random generator for sampling multiple rays per pixels
    let mut random = rand::thread_rng();
    let uniform_sampler = Uniform::from(0.0f32..1.0f32);

    // For each pixel in the viewport
    for y in 0..height {
        for x in 0..width {
            // Will be mutated for each sample
            let mut color: Color = Color::default();

            // Sample
            for _ in 0..sample_amount {
                // Screen space coordinates
                let u = (x as f32 + uniform_sampler.sample(&mut random)) / width as f32;
                let v = (y as f32 + uniform_sampler.sample(&mut random)) / height as f32;

                let ray = camera.get_ray(u, v);
                let _point = ray.point_at(2.0);

                color += color_ray(&ray, &world, 0);
            }

            // Average samples
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

#[allow(unused)]
fn create_world() -> HitableVec {
    let mut world = HitableVec::new();
    world.vec().push(Box::new(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    world.vec().push(Box::new(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    world.vec().push(Box::new(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.vec().push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.vec().push(Box::new(Sphere::new(
        Vec3(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    world
}

fn create_random_scene() -> HitableVec {
    let mut random = rand::thread_rng();
    let mut world = HitableVec::new();

    // Ground
    world.push_sphere(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new_boxed(Color::new(0.5, 0.5, 0.5)),
    );

    for a in -11..11 {
        for b in -11..11 {
            // Generate random center
            let center = Vec3(
                a as f32 + 0.9 * random.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * random.gen::<f32>(),
            );

            // Check that the little spheres are not under the big one in front
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                // Choose a random material
                let material: Box<dyn Material> = match random.gen::<f32>() {
                    // Diffuse
                    choose_mat if choose_mat < 0.8 => Lambertian::new_boxed(Color::new(
                        random.gen::<f32>() * random.gen::<f32>(),
                        random.gen::<f32>() * random.gen::<f32>(),
                        random.gen::<f32>() * random.gen::<f32>(),
                    )),
                    // Metal
                    choose_mat if choose_mat < 0.95 => Metal::new_boxed(
                        Color::new(
                            0.5 * (1.0 + random.gen::<f32>()),
                            0.5 * (1.0 + random.gen::<f32>()),
                            0.5 * (1.0 + random.gen::<f32>()),
                        ),
                        0.5 * (1.0 + random.gen::<f32>()),
                    ),
                    // Glass
                    _ => Dielectric::new_boxed(1.5),
                };

                world.push_sphere(center, 0.2, material);
            }
        }
    }

    world.push_sphere(Vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new_boxed(1.5));

    world.push_sphere(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new_boxed(Color::new(0.4, 0.2, 0.1)),
    );

    world.push_sphere(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Metal::new_boxed(Color::new(0.7, 0.6, 0.5), 0.0),
    );

    world
}
