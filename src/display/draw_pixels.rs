use graphics::{Context, Image};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{GlGraphics, Texture};
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::State;

pub(crate) fn draw_pixels(state: &State, d_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, texture: &mut Texture, context: Context, gl: &mut GlGraphics) {
    for pixel_num in 0..(EMU_WIDTH * EMU_HEIGHT) {
        let (x, y) = (pixel_num % EMU_WIDTH, pixel_num / EMU_WIDTH);

        if y % 2 == 0 {
            d_img.put_pixel(x, y, image::Rgba([255, 0, 0, 255]))
        } else {
            d_img.put_pixel(x, y, image::Rgba([0, 0, 255, 255]))
        }
    }
    texture.update(&d_img);
    Image::new().draw(texture, &context.draw_state, context.transform, gl);
}