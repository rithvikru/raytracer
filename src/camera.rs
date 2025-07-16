use crate::color::{Color, write_color};
use crate::hit::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, unit_vector};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    image_height: u64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    progress: ProgressBar,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 1200;
        let center = Point3::new(0.0, 0.0, 0.0);
        Self::new(aspect_ratio, image_width, center)
    }
}

impl Camera {
    fn new(aspect_ratio: f64, image_width: u64, center: Point3) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u64;
        if image_height < 1 {
            image_height = 1;
        }

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
            progress,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            self.progress.inc(1);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * (i as f64))
                    + (self.pixel_delta_v * (j as f64));
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);
                write_color(&pixel_color);
            }
        }

        self.progress.finish();
    }

    pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = unit_vector(r.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
