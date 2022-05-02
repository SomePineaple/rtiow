use crate::ray::Ray;
use camera::Camera;
use cgmath::{point3, vec3};
use rand::Rng;
use std::fs;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;

mod utils;
mod ray;
mod hittable;
mod camera;

const IMAGE_WIDTH: i32 = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 250;
const MAX_DEPTH: i32 = 10;

fn main() {
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(point3(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut output = String::new();
    output += format!("P3\n{} {}\n256\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_str();

    let mut rng = rand::thread_rng();

    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        println!("Lines left: {}, output size {}", j, output.len());
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);

            for _sample in 0..SAMPLES_PER_PIXEL {
                let mut rand: f64 = rng.gen();
                let u = (i as f64 + rand) / (IMAGE_WIDTH-1) as f64;
                rand = rng.gen();
                let v = (j as f64 + rand) / (IMAGE_HEIGHT-1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world, MAX_DEPTH, ray::DiffuseMode::LAMBERDIAN);
            }

            output += utils::color_str(pixel_color, SAMPLES_PER_PIXEL).as_str();
        }

        j -= 1;
    }

    fs::write("./test.ppm", output).expect("Failed to write file");
}
