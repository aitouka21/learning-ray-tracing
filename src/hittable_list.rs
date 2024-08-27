use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    #[allow(unused)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut interval = *ray_t;

        self.objects.iter().for_each(|object| {
            if let Some(hit_record) = object.hit(ray, &interval) {
                interval.max = hit_record.t;
                rec = Some(hit_record);
            }
        });
        rec
    }
}
