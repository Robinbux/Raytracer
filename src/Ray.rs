use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3, // Origin
    pub dir: Vec3,      // Direction
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            dir: direction,
        }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
