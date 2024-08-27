use rayon::prelude::*;
use std::ops::{Div, Mul, Rem};

use rand::prelude::*;

use crate::{
    color::{self, Color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    material::ScatterResult,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Debug)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: f64,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

pub struct Settings {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov: 20.0,
            lookfrom: Point3::new(-2.0, 2.0, 1.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_dist: 10.0,
        }
    }
}

impl Camera {
    pub fn new(
        Settings {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        }: Settings,
    ) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let image_height = match f64::from(image_width) / aspect_ratio {
            n if n > 1.0 => n as i32,
            _ => 1,
        };

        let center = lookfrom;
        let viewport_height = 2.0 * vfov.to_radians().div(2.0).tan() * focus_dist;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        let viewport_upper_left = center - focus_dist.mul(w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle.div(2.0).to_radians().tan());
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let pixel_samples_scale = f64::from(samples_per_pixel).recip();

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let colors = (0..self.image_width * self.image_height)
            .into_par_iter()
            .map(|n| {
                let j = n.div(self.image_width);
                let i = n.rem(self.image_width);
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                self.pixel_samples_scale * pixel_color
            })
            .collect::<Vec<_>>();

        println!("P3\n{0} {1}\n255", self.image_width, self.image_height);
        colors.iter().for_each(color::write);
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let i = f64::from(i);
        let j = f64::from(j);
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i + offset.x()) * self.pixel_delta_u
            + (j + offset.y()) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        let interval = Interval::new(0.001, f64::INFINITY);
        if let Some(hit_record) = world.hit(r, &interval) {
            if let Some(ScatterResult {
                attenuation,
                scattered,
            }) = hit_record.mat.scatter(r, &hit_record)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::default();
        }
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}
