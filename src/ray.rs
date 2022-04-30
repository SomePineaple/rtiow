use cgmath::{Point3, Vector3, vec3, point3, InnerSpace, dot};

pub type Color = Vector3<f64>;
pub type Point3d = Point3<f64>;

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
        let a = dot(self.dir, self.dir);
        let b = 2.0 * dot(oc, self.dir);
        let c = dot(oc, oc) - radius*radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return -1.0;
        }
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
