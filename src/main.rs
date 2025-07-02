mod color;
mod vec3;

use crate::{color::Color, color::write_color, vec3::Vec3};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let progress = ProgressBar::new(image_height);
    progress.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-")
    );

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.0
            );
            write_color(&pixel_color);
        }
        
        progress.inc(1);
    }

    progress.finish();
}
