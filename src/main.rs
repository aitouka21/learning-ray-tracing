use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::Material;
use sphere::Sphere;
use vec3::Point3;

fn main() {
    let mut world = HittableList::default();

    let material_ground = Rc::new(Material::lambertian(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Material::lambertian(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Material::metal(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Material::metal(Color::new(0.8, 0.6, 0.2), 1.0));

    {
        let center = Point3::new(0.0, -100.5, -1.0);
        let sphere = Sphere::new(center, 100.0, Rc::clone(&material_ground));
        world.add(Rc::new(sphere));
    }

    {
        let center = Point3::new(0.0, 0.0, -1.2);
        let sphere = Sphere::new(center, 0.5, Rc::clone(&material_center));
        world.add(Rc::new(sphere));
    }

    {
        let center = Point3::new(-1.0, 0.0, -1.0);
        let sphere = Sphere::new(center, 0.5, Rc::clone(&material_left));
        world.add(Rc::new(sphere));
    }

    {
        let center = Point3::new(1.0, 0.0, -1.0);
        let sphere = Sphere::new(center, 0.5, Rc::clone(&material_right));
        world.add(Rc::new(sphere));
    }

    let camera = Camera::default();
    camera.render(&world);
}
