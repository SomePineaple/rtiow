use crate::hittable::hittable_list::HittableList;
use crate::hittable::material::{Dielectric, Lambertian, Metal};
use crate::hittable::sphere::Sphere;
use crate::ray::Ray;
use camera::Camera;
use cgmath::Vector3;
use cgmath::{point3, vec3};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use utils::vec3_length_squared;
use std::collections::HashMap;

mod camera;
mod hittable;
mod ray;
mod utils;

const IMAGE_WIDTH: i32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const FOV: f64 = 25.0;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 512;
const MAX_DEPTH: i32 = 50;

const MAX_THREADS: i32 = 10;

struct Scanline {
    pub pixels: Vec<Vector3<f64>>,
    pub line_num: i32,
}

impl Scanline {
    fn new(pixels: Vec<Vector3<f64>>, line_num: i32) -> Self {
        Self { pixels, line_num }
    }

    fn to_string(&self) -> String {
        let mut output = String::new();

        for pixel in self.pixels.to_vec() {
            output += utils::color_str(pixel, SAMPLES_PER_PIXEL).as_str();
        }

        output
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let mut rng = StdRng::seed_from_u64(1577);

    world.add(Box::new(Sphere::new(
        point3(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(vec3(0.8, 0.8, 0.8))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let albedo = vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
            let choose_mat: f64 = rng.gen::<f64>();
            let center = point3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if vec3_length_squared(center - point3(4.0, 0.2, 0.0)).sqrt() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    let color = albedo / 2.0 + vec3(0.5, 0.5, 0.5);
                    let fuzz = rng.gen::<f64>() / 2.0;
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(color, fuzz)),
                    )));
                } else {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        point3(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        point3(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(vec3(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        point3(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)),
    )));

    return world;
}

fn main() {
    let look_from = point3(12.0, 2.0, 3.0);
    let look_at = point3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        FOV,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut output = String::new();
    output += format!("P3\n{} {}\n256\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_str();

    let (send, recv): (Sender<Scanline>, Receiver<Scanline>) = mpsc::channel();

    let mut threads: HashMap<i32, thread::JoinHandle<_>> = HashMap::new();

    let mut num_threads = 0;

    let mut lines: Vec<Scanline> = Vec::new();

    let mut j = IMAGE_HEIGHT - 1;
    loop {
        if num_threads < MAX_THREADS && j >= 0 {
            let line_num = j;
            let spx = SAMPLES_PER_PIXEL;

            let local_camera = camera.clone();
            let local_send = send.clone();
            println!("spawning {}", j);
            threads.insert(j, thread::spawn(move || {
                let world = random_scene();
                let mut pixels: Vec<Vector3<f64>> = Vec::new();
                let mut rng = rand::thread_rng();
                for i in 0..IMAGE_WIDTH {
                    let mut pixel_color = vec3(0.0, 0.0, 0.0);
                    for _sample in 0..spx {
                        let mut rand: f64 = rng.gen();
                        let u = (i as f64 + rand) / (IMAGE_WIDTH - 1) as f64;
                        rand = rng.gen();
                        let v = (line_num as f64 + rand) / (IMAGE_HEIGHT - 1) as f64;
                        let r = local_camera.get_ray(u, v);
                        pixel_color += r.color(&world, MAX_DEPTH);
                    }
                    pixels.push(pixel_color);
                }
                local_send.send(Scanline::new(pixels, line_num)).unwrap();
            }));

            j -= 1;
            num_threads += 1;
        }

        let recvd = recv.try_recv();
        if recvd.is_ok() {
            let scanline = recvd.unwrap();
            println!("finished {}", scanline.line_num);
            threads.remove(&scanline.line_num).unwrap().join().expect("failed to join thread");
            num_threads -= 1;
            lines.push(scanline);
        }

        if j < 0 && num_threads == 0 {
            break;
        }
    }

    lines.sort_by(|a, b| b.line_num.cmp(&a.line_num));

    for line in lines {
        output += &line.to_string();
    }

    fs::write("./test.ppm", output).expect("Failed to write file");
}
