use cgmath::{Point3, Vector3, point3, vec3, InnerSpace};

use crate::ray::Ray;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(look_from: Point3<f64>, look_at: Point3<f64>, vup: Vector3<f64>, fov: f64, aspect_ratio: f64) -> Self {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;
        Self {
            origin, lower_left_corner, horizontal, vertical
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
