use crate::hittable::{HitRecord, Hittable};
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<std::rc::Rc<dyn Hittable>>,
}

impl HittableList {
    #[allow(unused)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_tmax;

        self.objects.iter().for_each(|object| {
            if let Some(hit_record) = object.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = hit_record.t;
                rec = Some(hit_record);
            }
        });
        rec
    }
}
