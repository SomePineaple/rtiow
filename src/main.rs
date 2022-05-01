use crate::ray::Ray;
use cgmath::{point3, Point3, vec3};
use std::fs;
use crate::hittable::hittable_list::HittableList;
use crate::hittable::sphere::Sphere;

mod utils;
mod ray;
mod hittable;

const IMAGE_WIDTH: i32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

const ORIGIN: Point3<f64> = point3(0.0, 0.0, 0.0);

fn main() {
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        ORIGIN - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(point3(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(point3(0.0, -100.5, -1.0), 100.0)));

    let mut output = String::new();
    output += format!("P3\n{} {}\n256\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_str();
    let mut j = IMAGE_HEIGHT - 1;
    while j >= 0 {
        println!("Lines left: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                ORIGIN,
                lower_left_corner + u * horizontal + v * vertical - ORIGIN,
            );

            output += utils::color_str(r.color(&world)).as_str();
        }

        j -= 1;
    }

    fs::write("./test.ppm", output).expect("Failed to write file");
}
