use crate::hittable::{HitRecord, Hittable};
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, min: f64, max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest = max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, min, max) {
                if rec.t < closest {
                    closest = rec.t;
                    temp_rec = Some(rec);
                }
            }
        }

        return temp_rec;
    }

    fn box_clone(&self) -> Box<dyn Hittable> {
        let mut cloned = Self::new();

        for object in self.objects.as_slice() {
            cloned.add(object.box_clone());
        }

        Box::new(cloned)
    }
}
