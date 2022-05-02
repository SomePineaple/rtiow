use cgmath::{Vector3, vec3, dot, InnerSpace};
use rand::Rng;

pub fn to_byte_rgb(color: f64) -> u8 {
    (255.999 * color) as u8
}

pub fn rand_vec3_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = (rand_vec3() * 2.0) - vec3(1.0, 1.0, 1.0);
        if (p.x*p.x + p.y*p.y + p.z*p.z) >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn rand_vec3_in_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = rand_vec3_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn rand_normalized_vec3() -> Vector3<f64> {
    return rand_vec3_in_unit_sphere().normalize();
}

pub fn rand_vec3() -> Vector3<f64> {
    let mut rnd = rand::thread_rng();
    let x: f64 = rnd.gen();
    let y: f64 = rnd.gen();
    let z: f64 = rnd.gen();
    vec3(x, y, z)
}

pub fn clamp(num: f64, min: f64, max: f64) -> f64 {
    if min > num {
        return min;
    }
    if max < num {
        return max;
    }
    num
}

pub fn color_str(color: Vector3<f64>, samples_per_pixel: i32) -> String {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale*r).sqrt();
    g = (scale*g).sqrt();
    b = (scale*b).sqrt();
    return format!(
        "{} {} {}\n",
        to_byte_rgb(clamp(r, 0.0, 0.999)),
        to_byte_rgb(clamp(g, 0.0, 0.999)),
        to_byte_rgb(clamp(b, 0.0, 0.999))
    );
}
