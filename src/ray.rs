use cgmath::{Point3, Vector3, vec3, point3, InnerSpace, dot};
use crate::hittable::Hittable;
use crate::hittable::hittable_list::HittableList;

pub type Color = Vector3<f64>;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, dir: Vector3<f64>) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        return self.origin + t * self.dir;
    }

    pub fn color(&self, world: &HittableList) -> Color {
        if let Some(hit) = world.hit(*self, 0.0, f64::MAX) {
            return 0.5 * (hit.normal + vec3(1.0, 1.0, 1.0));
        }
        let t = 0.5 * (self.dir.normalize().y + 1.0);
        (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
    }
}
