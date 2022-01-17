
use pixel_canvas::{Canvas, Color, input::MouseState};

mod cpu;
mod bus;
mod ppu;
mod state;
mod cartridge;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.5];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const DARK_BLUE: [f32; 4] = [0.0, 0.0, 0.67, 1.0];

fn main() {
    let canvas = Canvas::new(512, 512)
        .title("IronNES")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    // The canvas will render for you at up to 60fps.
    canvas.render(|mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - mouse.x;
                let dy = y as i32 - mouse.y;
                let dist = dx * dx + dy * dy;
                *pixel = Color {
                    r: if dist < 128 * 128 { dy as u8 } else { 0 },
                    g: if dist < 128 * 128 { dx as u8 } else { 0 },
                    b: (x * y) as u8,
                }
            }
        }
    });
}
