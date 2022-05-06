use cgmath::{dot, Point3, Vector3};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::material::Material;
use crate::Ray;

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

pub struct Sphere {
    pub mat: Box<dyn Material>,
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, mat: Box<dyn Material>) -> Self {
        Self { mat, center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = dot(oc, r.dir);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        // Find the closest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root < min || max < root {
            root = (-half_b + sqrt_d) / a;
            if root < min || max < root {
                return None;
            }
        }

        let mut rec = HitRecord::new(&self.mat);
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return Some(rec);
    }
}
