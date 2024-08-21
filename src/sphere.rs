use crate::{
    hittable::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

pub struct Phantom;

impl Hittable for Phantom {
    fn hit(&self, _ray: &crate::ray::Ray, _ray_tmin: f64, _ray_tmax: f64) -> Option<HitRecord> {
        None
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = &self.center - ray.origin();
        let a = ray.direction().len_squared();
        let h = Vec3::dot(ray.direction(), &oc);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            };
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let rec = HitRecord::new(root, ray.at(root), ray, outward_normal);
        Some(rec)
    }
}
