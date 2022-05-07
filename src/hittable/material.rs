use cgmath::{InnerSpace, dot, vec3};

use super::HitRecord;
use crate::Ray;
use crate::ray::Color;
use crate::utils::{rand_normalized_vec3, rand_vec3_in_unit_sphere, vec3_near_zero, vec3_reflect, vec3_refract};

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Color, Ray, bool);
    fn box_clone(&self) -> Box<dyn Material>;
}

pub struct Lambertian {
    pub color: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let mut scatter_direction = rec.normal + rand_normalized_vec3();
        if vec3_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }
        (self.color, Ray::new(rec.p, scatter_direction), true)
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.color.clone()))
    }
}

pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            color,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let reflected = vec3_reflect(ray.dir.normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*rand_vec3_in_unit_sphere());
        (self.color, scattered, dot(scattered.dir, rec.normal) > 0.0)
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.color.clone(), self.fuzz))
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let refraction_ratio: f64;
        if rec.front_face {
            refraction_ratio = (1.0/self.ir);
        } else {
            refraction_ratio = self.ir;
        }
        let normalized_dir = ray.dir.normalize();
        let refracted = vec3_refract(normalized_dir, rec.normal, refraction_ratio);
        (vec3(1.0, 1.0, 1.0), Ray::new(rec.p, refracted), true)
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Dielectric::new(self.ir))
    }
}
