use cgmath::{InnerSpace, Vector3, vec3, dot};

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
        if self.hit_sphere(vec3(0.0, 0.0, -1.0), 0.5) {
            return vec3(1.0, 0.0, 0.0);
        }
        let t = 0.5 * (self.dir.normalize().y + 1.0);
        (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: Point3, radius: f64) -> bool {
        let oc = self.origin - center;
        let a = dot(self.dir, self.dir);
        let b = 2.0 * dot(oc, self.dir);
        let c = dot(oc, oc) - radius*radius;
        let discriminant = b*b - 4.0*a*c;
        return discriminant > 0.0;
    }
}
