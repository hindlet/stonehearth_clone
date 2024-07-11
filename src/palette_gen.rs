use std::{fs::File, io::Write};

use image::{io::Reader, GenericImageView, ImageFormat, RgbImage};




const WIDTH: u32 = 16;
const HEIGHT: u32 = 16;


pub fn gen_palette() {
    let img = Reader::open("assets/atari_256_palette.png").unwrap().decode().unwrap();
    let mut img_new = RgbImage::new(WIDTH, HEIGHT);
    let mut line_buffer = Vec::new();

    let (width, height) = img.dimensions();

    let x_step = width as f32 / WIDTH as f32;
    let initial_x = (x_step / 2.0) as u32;
    let y_step = height as f32 / HEIGHT as f32;
    let initial_y = (y_step / 2.0) as u32;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let colour = img.get_pixel(initial_x + (x as f32 * x_step) as u32, initial_y + (y as f32 * y_step) as u32).0;
            img_new.get_pixel_mut(x, y).0 = [colour[0], colour[1], colour[2]];
            writeln!(&mut line_buffer, "({0}, {1}, {2}, {3}) => Some({4}),", colour[0], colour[1], colour[2], colour[3], y * WIDTH + x).unwrap();
        }
    }

    let mut colour_match_case_file = File::create("assets/colour_match_case.txt").unwrap();
    colour_match_case_file.write_all(&line_buffer).unwrap();

    img_new.save_with_format("assets/palette.png", ImageFormat::Png).unwrap();
}