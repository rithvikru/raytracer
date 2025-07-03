use crate::{
    vec3::{Point3, Vec3, dot},
    ray::Ray, 
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
};

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        front_face = dot(r.direction(), outward_normal) < 0.0;
        normal = if front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool;
};
