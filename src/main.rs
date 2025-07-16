mod camera;
mod color;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hit::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();

    cam.render(&world);
}
