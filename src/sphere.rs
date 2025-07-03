use crate::{
    hit::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
    ray::Ray,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, fmax(0.0, radius) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool {
        let oc = center - r.origin();

        let a = r.direction().length_squared();
        let h = dot(&r.direction(), &oc);
        let c = oc.length_squared() - radius * radius;

        let discriminant = (h * h) - (a * c);
        if discriminant < 0.0 {
            return false;
        } 
        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if root <= ray_tmin or ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin or ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
