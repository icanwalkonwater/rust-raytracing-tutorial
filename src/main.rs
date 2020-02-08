use rust_raytracing_tutorial::Vec3;

fn main() {
    let width = 200;
    let height = 100;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for y in (0..height).rev() {
        for x in 0..width {
            let mut color = Vec3::new(x as f32 / width as f32, y as f32 / height as f32, 0.2);

            color *= 255.99;

            println!("{:.0} {:.0} {:.0}", color.r(), color.g(), color.b());
        }
    }
}
