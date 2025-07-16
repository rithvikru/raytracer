use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    const INTENSITY: Interval = Interval::new(0.000, 0.999);
    let rbyte = (INTENSITY.clamp(r) * 256.0) as u8;
    let gbyte = (INTENSITY.clamp(g) * 256.0) as u8;
    let bbyte = (INTENSITY.clamp(b) * 256.0) as u8;

    println!("{rbyte} {gbyte} {bbyte}");
}
