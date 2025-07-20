use crate::vec3::{dot, random_unit_vector, reflect, refract, unit_vector};
use crate::{color::Color, hit::HitRecord, ray::Ray};
use rand::Rng;

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.p, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut reflected = reflect(ray_in.direction(), hit_record.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.p, reflected),
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = dot(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0_f64 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let mut rng = rand::rng();
        let direction = if cannot_refract || reflectance(cos_theta, ri) > rng.random_range(0.0..1.0) {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, ri)
        };

        let scattered = Ray::new(hit_record.p, direction);

        Some(ScatterResult {
            attenuation: attenuation,
            scattered: scattered,
        })
    }

}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
