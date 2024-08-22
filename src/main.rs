use std::{
    io::{self, Write},
    ops::Div,
    rc::Rc,
};

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use hittable::Hittable;
use hittable_list::HittableList;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let interval = Interval::new(0.0, f64::INFINITY);
    if let Some(hit_record) = world.hit(r, &interval) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    #[allow(clippy::cast_possible_truncation)]
    let mut image_height = f64::from(image_width).div(aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    let mut world = HittableList::default();

    let s1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    world.add(Rc::new(s1));
    world.add(Rc::new(s2));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut write_buffer = io::stdout();
    writeln!(write_buffer, "P3\n{image_width} {image_height}\n255")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let i = f64::from(i);
            let j = f64::from(j);

            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let c = ray_color(&ray, &world);
            color::write(&mut write_buffer, &c)?;
        }
    }
    write_buffer.flush()
}
