use rand::prelude::*;
use std::io::Write;

use crate::{
    color::{self, Color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    material::ScatterResult,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(16.0 / 9.0, 400, 100, 50)
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        let image_height = match f64::from(image_width) / aspect_ratio {
            n if n > 1.0 => n as i32,
            _ => 1,
        };

        let center = Point3::default();
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / f64::from(samples_per_pixel);

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render(&self, world: &HittableList) -> std::io::Result<()> {
        let mut write_buffer = std::io::stdout();
        writeln!(
            write_buffer,
            "P3\n{0} {1}\n255",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                let c = self.pixel_samples_scale * pixel_color;
                color::write(&mut write_buffer, &c)?;
            }
        }
        write_buffer.flush()
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

        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
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

            Color::default();
        }
        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
