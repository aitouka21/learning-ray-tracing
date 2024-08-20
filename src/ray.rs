use crate::vec3::{Point3, Vec3};

#[allow(unused)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[allow(unused)]
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[allow(unused)]
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    #[allow(unused)]
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    #[allow(unused)]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
