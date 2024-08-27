use std::sync::Arc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::{Camera, Settings};
use color::Color;
use hittable_list::HittableList;
use material::Material;
use rand::Rng;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::default();

    for a in -11..11 {
        for b in -11..11 {
            let mut rng = rand::thread_rng();
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                f64::from(a) + rng.gen_range(0.0..0.9),
                0.2,
                f64::from(b) + rng.gen_range(0.0..0.9),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let mat = if choose_mat < 0.8 {
                    let c1 = Color::new(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    );
                    let c2 = Color::new(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    );
                    let albedo = c1 * c2;
                    Arc::new(Material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    let albedo = Color::new(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    );
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Material::metal(albedo, fuzz))
                } else {
                    Arc::new(Material::dielectric(1.5))
                };
                let sphere = Sphere::new(center, 0.2, mat);
                world.add(Box::new(sphere));
            }
        }
    }

    {
        let material = Arc::new(Material::lambertian(Color::new(0.5, 0.5, 0.5)));
        let center = Point3::new(0.0, -1000.0, 0.0);
        let sphere = Sphere::new(center, 1000.0, material);
        world.add(Box::new(sphere));
    }

    {
        let material = Arc::new(Material::dielectric(1.5));
        let center = Point3::new(0.0, 1.0, 0.0);
        let sphere = Sphere::new(center, 1.0, material);
        world.add(Box::new(sphere));
    }

    {
        let material = Arc::new(Material::lambertian(Color::new(0.4, 0.2, 0.1)));
        let center = Point3::new(-4.0, 1.0, 0.0);
        let sphere = Sphere::new(center, 1.0, material);
        world.add(Box::new(sphere));
    }

    {
        let material = Arc::new(Material::metal(Color::new(0.7, 0.6, 0.5), 0.0));
        let center = Point3::new(4.0, 1.0, 0.0);
        let sphere = Sphere::new(center, 1.0, material);
        world.add(Box::new(sphere));
    }

    let settings = Settings {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    };

    let camera = Camera::new(settings);
    camera.render(&world);
}
