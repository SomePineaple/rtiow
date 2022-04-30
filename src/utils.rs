use cgmath::{InnerSpace, Vector3, vec3};

pub type Point3 = Vector3<f64>;
pub type Direction = Vector3<f64>;
pub type Color = Vector3<f64>;

pub fn to_byte_rgb(color: f64) -> u8 {
    (255.999 * color) as u8
}

pub fn color_str(color: Vector3<f64>) -> String {
    return format!(
        "{} {} {}\n",
        to_byte_rgb(color.x),
        to_byte_rgb(color.y),
        to_byte_rgb(color.z)
    );
}

pub struct Ray {
    origin: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3, dir: Direction) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.dir;
    }

    pub fn color(&self) -> Color {
        let t = 0.5 * (self.dir.normalize().y + 1.0);
        (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
    }
}
