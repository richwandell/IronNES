extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

const EMU_WIDTH: u32 = 256;
const EMU_HEIGHT: u32 = 240;

#[derive(Debug)]
pub(crate) struct IColor {
    value: [u8; 4]
}

impl From<[u8; 4]> for IColor {

    fn from(item: [u8; 4]) -> IColor {
        IColor {
            value: item
        }
    }
}

impl From<IColor> for [f32; 4] {
    fn from(item: IColor) -> Self {
        [item.value[0] as f32 / 255.0,
            item.value[1] as f32 / 255.0,
            item.value[2] as f32 / 255.0,
            item.value[3] as f32 / 255.0]
    }
}

#[test]
fn test() {
    use graphics::types::Color;
    let c: Color = IColor::from([128, 255, 30, 255]).into();
    assert_eq!(c, [0.5019608, 1.0, 0.11764706, 1.0]);
}

mod draw_debug;
mod draw_pixels;
pub mod display;
pub mod display_snake;
pub mod display_nes;
pub mod display_debug;