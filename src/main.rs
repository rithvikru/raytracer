fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for r in 0..image_height {
        for c in 0..image_width {
            let red = c as f32 / (image_width - 1) as f32;
            let green = r as f32 / (image_height - 1) as f32;
            let blue = 0.0;

            let i_red = (255.999 * red) as u8;
            let i_green = (255.999 * green) as u8;
            let i_blue = (255.999 * blue) as u8;

            println!(
                "{} {} {}",
                i_red, i_green, i_blue
            );
        }
    }
}
