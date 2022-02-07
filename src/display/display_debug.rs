use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::rc::Rc;
use graphics::{clear};
use image::{ImageBuffer, Rgba};
use image::png::CompressionType::Default;
use opengl_graphics::{GlyphCache, Texture, TextureSettings};
use piston::{Button, Key, PressEvent};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use pixel_canvas::input::Event;
use crate::display::display::{Game, get_scaled_context, NesSystem};
use crate::display::draw_debug::draw_debug;
use crate::display::draw_pixels::draw_pixels;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::{Cpu, Ppu, State};
use crate::{advance, create_system};

struct Debug {
    data: HashMap<String, HashMap<u32, String>>,
    visible_pages: Vec<u16>,
}

pub struct NesDebug(NesSystem, Debug);

impl NesDebug {

    pub fn new(
        state: Rc<RefCell<State>>,
        cpu: Rc<RefCell<Cpu>>,
        ppu: Rc<RefCell<Ppu>>,
        visible_pages: Vec<u16>
    ) -> NesDebug {
        NesDebug(
            NesSystem::new(state, cpu, ppu),
            Debug {
                data: HashMap::default(),
                visible_pages
            }
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

    fn render(&mut self,
              args: RenderArgs,
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
                draw_debug(&*state, &*cpu, c, &mut glyphs, &disassembly, gl, visible_pages);
            });
        }
        {
            self.set_disassembly(disassembly);
        }
    }
}

impl Game for NesDebug {
    fn start(&mut self) {
        let settings = EventSettings {
            max_fps: 60,
            ups: 1000,
            swap_buffers: true,
            bench_mode: false,
            lazy: false,
            ups_reset: 2,
        };
        let mut events = Events::new(settings);
        let mut d_img = ImageBuffer::from_fn(EMU_WIDTH, EMU_HEIGHT, |x, y| {
            image::Rgba([255, 255, 255, 255])
        });
        let mut texture = Texture::from_image(&d_img, &TextureSettings::new());
        // Main loop
        let mut running = false;
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("junk/debug.csv")
            .unwrap();

        {
            let mut cpu = self.0.cpu.as_ref().borrow_mut();
            let mut ppu = self.0.ppu.as_ref().borrow_mut();
            while cpu.pc != 0xEB9e {
                let write_string = hex::encode(&cpu.pc.to_be_bytes());
                if let Err(e) = writeln!(file, "{}", write_string.to_uppercase()) {
                    eprintln!("Couldn't write to file: {}", e);
                }
                let _ = advance(&mut ppu, &mut cpu);
            }
        }

        while let Some(e) = events.next(&mut self.0.window) {
            if let Some(args) = e.press_args() {
                match args {
                    Button::Keyboard(key) => {
                        if key.eq(&Key::Space) {
                            running = !running;
                        } else {
                            let mut ppu = self.0.ppu.as_ref().borrow_mut();
                            let mut cpu = self.0.cpu.as_ref().borrow_mut();
                            let write_string = hex::encode(&cpu.pc.to_be_bytes());
                            if let Err(e) = writeln!(file, "{}", write_string.to_uppercase()) {
                                eprintln!("Couldn't write to file: {}", e);
                            }

                            let _ = advance(&mut ppu, &mut cpu);
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
                    let write_string = hex::encode(&cpu.pc.to_be_bytes());
                    if let Err(e) = writeln!(file, "{}", write_string.to_uppercase()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }

                    let _ = advance(&mut ppu, &mut cpu);
                }
            }
        }
    }
}
