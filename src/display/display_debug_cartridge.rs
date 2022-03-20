use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::rc::Rc;

use graphics::{clear, Image, Transformed};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{GlyphCache, Texture, TextureSettings};
use piston::{Button, Key, PressEvent};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};

use crate::{Cpu, Ppu, State};
use crate::advance;
use crate::bus::system_clock;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::display::display::{Game, get_scaled_context, NesSystem};
use crate::display::draw_debug::{draw_cart_debug, draw_debug};

struct Debug {
    data: HashMap<String, HashMap<u32, String>>,
    visible_pages: Vec<u16>,
    event_settings: EventSettings
}

pub struct NesDebugCartridge(NesSystem, Debug);

impl NesDebugCartridge {
    pub fn new(
        state: Rc<RefCell<State>>,
        cpu: Rc<RefCell<Cpu>>,
        ppu: Rc<RefCell<Ppu>>,
        visible_pages: Vec<u16>,
        event_settings: EventSettings,
    ) -> NesDebugCartridge {
        NesDebugCartridge(
            NesSystem::new(state, cpu, ppu, EMU_WIDTH, EMU_HEIGHT, 300),
            Debug {
                data: HashMap::default(),
                visible_pages,
                event_settings
            },
        )
    }

    fn get_disassembly(&mut self) -> HashMap<u32, String> {
        if let Some(items) = self.1.data.remove("disassembly") {
            return items;
        } else {
            let disassembly = self.0.cpu.as_ref().borrow_mut().disassemble();
            return disassembly;
        }
    }

    fn set_disassembly(&mut self, disassembly: HashMap<u32, String>) {
        self.1.data.insert("disassembly".to_string(), disassembly);
    }

    fn render(
        &mut self, args: RenderArgs,
        mut d_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        mut texture: &mut Texture
    ) {
        let disassembly = self.get_disassembly();

        {
            let state = self.0.state.as_ref().borrow();
            let cpu = self.0.cpu.as_ref().borrow();
            let visible_pages = &self.1.visible_pages;
            let mut glyphs: GlyphCache = GlyphCache::new("assets/PixelEmulator-xq08.ttf", (), TextureSettings::new()).unwrap();

            self.0.gl.draw(args.viewport(), |c, gl| {
                //Clear the screen
                clear([0.0, 0.0, 1.0, 1.0], gl);
                draw_cart_debug(&*state, &*cpu, c, &mut glyphs, &disassembly, gl, visible_pages);

                let size = c.get_view_size();
                let x_scaler = (size[0] - 300.0) / EMU_WIDTH as f64;
                let y_scaler = size[1] / EMU_HEIGHT as f64;
                let context = c.scale(x_scaler, y_scaler);

                // let context = get_scaled_context(c);


                for row in 0..EMU_HEIGHT {
                    for col in 0..EMU_WIDTH {
                        d_img.put_pixel(col, row, state.screen[row as usize][col as usize]);
                    }
                }

                texture.update(&d_img);
                Image::new().draw(texture, &context.draw_state, context.transform, gl);
            });
        }
        {
            self.set_disassembly(disassembly);
        }
    }
}

impl Game for NesDebugCartridge {
    fn start(&mut self) {
        let settings = self.1.event_settings;
        let mut events = Events::new(settings);
        let mut d_img = ImageBuffer::from_fn(EMU_WIDTH, EMU_HEIGHT, |x, y| {
            image::Rgba([255, 255, 255, 255])
        });
        let mut texture = Texture::from_image(&d_img, &TextureSettings::new());
        // Main loop
        let mut running = false;
        let mut updated = false;

        while let Some(e) = events.next(&mut self.0.window) {
            if let Some(args) = e.press_args() {
                match args {
                    Button::Keyboard(key) => {
                        if key.eq(&Key::Space) {
                            running = !running;
                        } else {
                            let mut ppu = self.0.ppu.as_ref().borrow_mut();
                            let mut cpu = self.0.cpu.as_ref().borrow_mut();
                            let _ = system_clock(&mut ppu, &mut cpu);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(args) = e.render_args() {
                self.render(args, &mut d_img, &mut texture);
            }

            if let Some(_args) = e.update_args() {
                if running {
                    let mut ppu = self.0.ppu.as_ref().borrow_mut();
                    let mut cpu = self.0.cpu.as_ref().borrow_mut();
                    let _ = system_clock(&mut ppu, &mut cpu);
                    updated = true;
                }
            }
        }
    }
}
