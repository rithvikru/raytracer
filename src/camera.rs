use crate::color::{write_color, Color};
use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    image_height: u64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u64,
    pixel_samples_scale: f64,
    max_depth: u64,
    progress: ProgressBar,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let samples_per_pixel = 100;
        let max_depth = 50;
        let center = Point3::new(0.0, 0.0, 0.0);
        Self::new(
            aspect_ratio,
            image_width,
            center,
            samples_per_pixel,
            max_depth,
        )
    }
}

impl Camera {
    fn new(
        aspect_ratio: f64,
        image_width: u64,
        center: Point3,
        samples_per_pixel: u64,
        max_depth: u64,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u64;
        if image_height < 1 {
            image_height = 1;
        }

        let pixel_samples_scale = 1.0 / (samples_per_pixel as f64);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let progress = ProgressBar::new(image_height);
        progress.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            progress,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            self.progress.inc(1);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&(pixel_color * self.pixel_samples_scale));
            }
        }

        self.progress.finish();
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(
            rng.random_range(-0.5..0.5),
            rng.random_range(-0.5..0.5),
            0.0,
        )
    }

    fn ray_color(&self, r: &Ray, depth: u64, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter_result) = rec.mat.scatter(r, &rec) {
                return scatter_result.attenuation
                    * self.ray_color(&scatter_result.scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(r.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
