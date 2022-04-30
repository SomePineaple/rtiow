use cgmath::{Point3, Vector3, vec3, point3, InnerSpace, dot};

pub type Color = Vector3<f64>;
pub type Point3d = Point3<f64>;

trait Length {
    fn length_squared(&self) -> f64;
    fn length(&self) -> f64;
}

impl Length for Vector3<f64> {
    fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub struct Ray {
    origin: Point3d,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3d, dir: Vector3<f64>) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3d {
        return self.origin + t * self.dir;
    }

    pub fn color(&self) -> Color {
        let mut t = self.hit_sphere(point3(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - point3(0.0, 0.0, -1.0)).normalize();
            return 0.5 * vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }
        t = 0.5 * (self.dir.normalize().y + 1.0);
        (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: Point3d, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.dir.length_squared();
        let half_b = dot(oc, self.dir);
        let c = oc.length_squared() - radius*radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return -1.0;
        }
        (-half_b - discriminant.sqrt()) / a
    }
}
