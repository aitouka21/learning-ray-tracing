use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = &self.center - ray.origin();
        let a = ray.direction().len_squared();
        let h = Vec3::dot(ray.direction(), &oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);
        let root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            };
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let rec = HitRecord::new(
            root,
            ray.at(root),
            ray,
            outward_normal,
            Rc::clone(&self.mat),
        );
        Some(rec)
    }
}
