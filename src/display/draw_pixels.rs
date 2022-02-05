use graphics::{Context, Image};
use graphics::types::Color;
use image::{ImageBuffer, Rgba};
use opengl_graphics::{GlGraphics, Texture};
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::{COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GRAY, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, COLOR_YELLOW, State};

fn color(byte: u8) -> [u8; 4] {
    match byte {
        0 => COLOR_BLACK,
        1 => COLOR_WHITE,
        2 | 9 => COLOR_GRAY,
        3 | 10 => COLOR_RED,
        4 | 11 => COLOR_GREEN,
        5 | 12 => COLOR_BLUE,
        6 | 13 => COLOR_MAGENTA,
        7 | 14 => COLOR_YELLOW,
        _ => COLOR_CYAN,
    }
}

pub(crate) fn draw_pixels(state: &State, d_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, texture: &mut Texture, context: Context, gl: &mut GlGraphics) {

    let start = 0x0200; //512
    let end = 0x600; //1536

    for byte_index in start..end {
        let y = (byte_index - 512) / 32;
        let x = (byte_index - 512) % 32;

        let color = color(state.cpu_ram[byte_index as usize]);
        d_img.put_pixel(x, y, image::Rgba(color))
    }

    // for pixel_num in 0..(EMU_WIDTH * EMU_HEIGHT) {
    //     let (x, y) = (pixel_num % EMU_WIDTH, pixel_num / EMU_WIDTH);
    //
    //     let color = color(state.cpu_ram[start+pixel_num as usize]);
    //     d_img.put_pixel(x, y, image::Rgba(color))
    //
    //     // if y % 2 == 0 {
    //     //     d_img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]))
    //     // } else {
    //     //     d_img.put_pixel(x, y, image::Rgba([0, 0, 255, 255]))
    //     // }
    // }
    texture.update(&d_img);
    Image::new().draw(texture, &context.draw_state, context.transform, gl);
}

#[test]
fn random_test() {
    let start = 0x0200; //512
    let end = 0x600; //1536

    for byte_index in start..end {
        let y = (byte_index - 512) / 32;
        let x = (byte_index - 512) % 32;

        println!("{}, {}", y, x);
    }
}