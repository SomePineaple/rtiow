pub mod sphere;
pub mod hittable_list;
pub mod material;

use cgmath::{dot, Point3, point3, vec3, Vector3};
use crate::hittable::material::Material;
use crate::Ray;

pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub mat: Box<dyn Material>,
    front_face: bool,
}

impl HitRecord {
    pub fn new(mat: &Box<dyn Material>) -> Self {
        Self {
            p: point3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            t: 0.0,
            mat: mat.box_clone(),
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vector3<f64>) {
        self.front_face = dot(ray.dir, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, min: f64, max: f64) -> Option<HitRecord>;
    fn box_clone(&self) -> Box<dyn Hittable>;
}
