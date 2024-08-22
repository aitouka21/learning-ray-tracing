use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::default();

    let s1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    world.add(Rc::new(s1));
    world.add(Rc::new(s2));

    let camera = Camera::default();
    camera.render(&world)
}
