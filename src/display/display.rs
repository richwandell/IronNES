use std::borrow::Borrow;
use std::cell::{RefCell};
use std::collections::HashMap;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use graphics::{clear, Transformed};
use graphics::rectangle::square;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use graphics::math::Scalar;
use graphics::types::Color;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use graphics::Context;
use piston::{Button, Key, MouseButton, PressEvent};
use crate::display::draw_debug::draw_debug;
use crate::display::draw_pixels::draw_pixels;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::{Cpu, State};

pub struct Display {
    window: Window,
    gl: GlGraphics,
    state: Rc<RefCell<State>>,
    cpu: Rc<RefCell<Cpu>>,
    debug: bool
}

fn get_scaled_context(c: Context) -> Context {
    let size = c.get_view_size();
    let x_scaler = size[0] / EMU_WIDTH as f64;
    let y_scaler = size[1] / EMU_HEIGHT as f64;
    c.scale(x_scaler, y_scaler)
}

impl Display {

    pub fn new(state: Rc<RefCell<State>>, cpu: Rc<RefCell<Cpu>>, debug: bool) -> Display {
        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new("IronNES", [EMU_WIDTH * 3, EMU_HEIGHT * 3])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut gl  = GlGraphics::new(opengl);

        Display {
            window,
            gl,
            state,
            cpu,
            debug
        }
    }

    fn render(&mut self,
              args: RenderArgs,
              mut d_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
              mut texture: &mut Texture,
              disassembly: &mut HashMap<u16, String>
    ) {
        let state = self.state.as_ref().borrow();
        let cpu = self.cpu.as_ref().borrow();

        let mut glyphs: GlyphCache = GlyphCache::new("assets/PixelEmulator-xq08.ttf", (), TextureSettings::new()).unwrap();

        let debug = self.debug;
        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen
            clear([0.0, 0.0, 1.0, 1.0], gl);

            let context = get_scaled_context(c);

            if  debug {
                draw_debug(&*state, &*cpu, c, &mut glyphs, disassembly, gl);
            } else {
                draw_pixels(&*state, d_img, texture, context, gl);
            }
        });
    }

    pub fn start<U, F>(&mut self,settings: EventSettings, update: U, button: F)
        where U: Fn(UpdateArgs), F: Fn(Button) {
        let mut events = Events::new(settings);
        let mut d_img = ImageBuffer::from_fn(EMU_WIDTH, EMU_HEIGHT, |x, y| {
            image::Rgba([255, 255, 255, 255])
        });

        let mut texture = Texture::from_image(&d_img, &TextureSettings::new());
        let mut disassembly = self.cpu.as_ref().borrow_mut().disassemble();
        // Main loop
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.press_args() {
                button(args);
            }
            if let Some(args) = e.render_args() {
                self.render(args, &mut d_img, &mut texture, &mut disassembly);
            } else if let Some(args) = e.update_args() {
                update(args);
            }
        }
    }
}