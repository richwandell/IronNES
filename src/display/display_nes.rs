use std::cell::RefCell;
use std::rc::Rc;
use glutin_window::OpenGL;
use graphics::{clear};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{GlGraphics, GlyphCache, Texture, TextureSettings};
use piston::{PressEvent, WindowSettings};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use crate::display::display::{Game, get_scaled_context, NesSystem};
use crate::display::draw_pixels::draw_pixels;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::{Cpu, State, Ppu};
use crate::{advance};

impl NesSystem {
    pub fn new(
        state: Rc<RefCell<State>>,
        cpu: Rc<RefCell<Cpu>>,
        ppu: Rc<RefCell<Ppu>>
    ) -> NesSystem {

        let opengl = OpenGL::V3_2;

        let window = WindowSettings::new("IronNES", [EMU_WIDTH * 3, EMU_HEIGHT * 3])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let gl  = GlGraphics::new(opengl);

        NesSystem {
            window,
            gl,
            state,
            cpu,
            ppu
        }
    }

    fn render(&mut self,
              args: RenderArgs,
              mut d_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
              mut texture: &mut Texture
    ) {
        let state = self.state.as_ref().borrow();
        let cpu = self.cpu.as_ref().borrow();

        let mut glyphs: GlyphCache = GlyphCache::new("assets/PixelEmulator-xq08.ttf", (), TextureSettings::new()).unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen
            clear([0.0, 0.0, 1.0, 1.0], gl);

            let context = get_scaled_context(c);

            draw_pixels(&*state, d_img, texture, context, gl);
        });
    }
}

impl Game for NesSystem {
    fn start(&mut self){
        let mut events = Events::new(EventSettings::new());
        let mut d_img = ImageBuffer::from_fn(EMU_WIDTH, EMU_HEIGHT, |x, y| {
            image::Rgba([255, 255, 255, 255])
        });
        let mut texture = Texture::from_image(&d_img, &TextureSettings::new());
        // Main loop

        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.press_args() {
                let mut ppu = self.ppu.as_ref().borrow_mut();
                let mut cpu = self.cpu.as_ref().borrow_mut();

                if let Ok(_) = advance(&mut ppu, &mut cpu) {
                    println!("{}", "clock ok");
                } else {
                    println!("{}", "clock not ok");
                }
            }

            if let Some(args) = e.render_args() {
                self.render(args, &mut d_img, &mut texture);
            }
        }
    }
}