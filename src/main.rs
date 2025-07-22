mod camera;
mod color;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hit::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Point3;
use rand::Rng;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();
    let mut rng = rand::rng();

    let ground_mat = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let center = Point3::new(a as f64 + 0.9 * rng.random_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.random_range(0.0..1.0));

            let sphere_mat: Rc<dyn crate::material::Material> = if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random_from_range(0.0, 1.0) * Color::random_from_range(0.0, 1.0);
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_from_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    Rc::new(Dielectric::new(1.5))
                }
            } else {
                continue;
            };

            world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(
        Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1),
    ));

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(
        Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2),
    ));

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(
        Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3),
    ));

    let mut cam = Camera::default();

    cam.render(&world);
}
