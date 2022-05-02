use cgmath::{Point3, Vector3, vec3, point3, InnerSpace};
use crate::hittable::Hittable;
use crate::hittable::hittable_list::HittableList;
use crate::utils::{rand_normalized_vec3, rand_vec3_in_hemisphere};

pub type Color = Vector3<f64>;

pub enum DiffuseMode {
    LAMBERDIAN, HEMISPHERE
}

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

    pub fn color(&self, world: &HittableList, depth: i32, mode: DiffuseMode) -> Color {
        if depth <= 0 {
            return vec3(0.0, 0.0, 0.0);
        }
        if let Some(hit) = world.hit(*self, 0.001, f64::MAX) {
            let mut target: Point3<f64> = point3(0.0, 0.0, 0.0);
            match mode {
                DiffuseMode::HEMISPHERE => target = hit.p + hit.normal + rand_normalized_vec3(),
                DiffuseMode::LAMBERDIAN => target = hit.p + rand_vec3_in_hemisphere(hit.normal),
            }
            return 0.5 * Ray::new(hit.p, target - hit.p).color(world, depth-1, mode);
        }
        let t = 0.5 * (self.dir.normalize().y + 1.0);
        (1.0-t)*vec3(1.0, 1.0, 1.0) + t*vec3(0.5, 0.7, 1.0)
    }
}
