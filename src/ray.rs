use crate::vec3::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3<f64>) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3<f64> {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
