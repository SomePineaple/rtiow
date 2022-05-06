use cgmath::{Point3, Vector3, vec3, InnerSpace};
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

    pub fn color(&self, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return vec3(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit(*self, 0.001, f64::MAX) {
            let target: Point3<f64>;
            let (attenuation, scattered, keep_going) = hit.mat.scatter(self, &hit);
            if keep_going {
                let other_color = scattered.color(world, depth-1);
                return vec3(attenuation.x * other_color.x, attenuation.y * other_color.y, attenuation.z * other_color.z);
            }
            return vec3(0.0, 0.0, 0.0);
        }
        let t = 0.5 * (self.dir.normalize().y + 1.0);
        (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
    }
}
