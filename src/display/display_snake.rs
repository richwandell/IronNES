use graphics::{clear, Image, Transformed};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{Texture, TextureSettings};
use piston::{Button, Key, PressEvent};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use crate::display::display::{Game, NesSystem};
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use crate::{COLOR_BLACK, COLOR_BLUE, COLOR_CYAN, COLOR_GRAY, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, COLOR_YELLOW, Cpu, Ppu, State};
use std::cell::RefCell;
use std::rc::Rc;
use rand::{Rng, thread_rng};
use crate::{advance};
use crate::bus::cpu_write;

pub struct SnakeGame(NesSystem);

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

impl SnakeGame {

    pub fn new(state: Rc<RefCell<State>>, cpu: Rc<RefCell<Cpu>>, ppu: Rc<RefCell<Ppu>>) -> SnakeGame {
        SnakeGame(NesSystem::new(state, cpu, ppu))
    }

    fn render(&mut self,
              args: RenderArgs,
              d_img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
              texture: &mut Texture
    ) {
        let state = self.0.state.as_ref().borrow();

        self.0.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen
            clear([0.0, 0.0, 1.0, 1.0], gl);

            let size = c.get_view_size();
            let x_scaler = size[0] / 32 as f64;
            let y_scaler = size[1] / 32 as f64;
            let context = c.scale(x_scaler, y_scaler);

            // let context = get_scaled_context(c);

            let start = 0x0200; //512
            let end = 0x600; //1536

            for byte_index in start..end {
                let y = (byte_index - 512) / 32;
                let x = (byte_index - 512) % 32;

                let color = color(state.cpu_ram[byte_index as usize]);
                d_img.put_pixel(x, y, image::Rgba(color))
            }
            texture.update(&d_img);
            Image::new().draw(texture, &context.draw_state, context.transform, gl);
        });
    }

    fn key_press(&mut self, args: Button) {
        let mut state = self.0.state.as_ref().borrow_mut();
        match args {
            Button::Keyboard(key) => {
                println!("{:?}", key);
                if key.eq(&Key::Up) {
                    cpu_write(&mut state, 0xff, 0x77);
                } else if key.eq(&Key::Down) {
                    cpu_write(&mut state, 0xff, 0x73);
                } else if key.eq(&Key::Left) {
                    cpu_write(&mut state, 0xff, 0x61);
                } else if key.eq(&Key::Right) {
                    cpu_write(&mut state, 0xff, 0x64);
                }
            }
            _ => {}
        }
    }
}

impl Game for SnakeGame {
    fn start(&mut self) {
        let mut events = Events::new(EventSettings {
            max_fps: 60,
            ups: 7000,
            swap_buffers: true,
            bench_mode: false,
            lazy: false,
            ups_reset: 0,
        });
        let mut d_img = ImageBuffer::from_fn(EMU_WIDTH, EMU_HEIGHT, |x, y| {
            image::Rgba([255, 255, 255, 255])
        });
        let mut texture = Texture::from_image(&d_img, &TextureSettings::new());
        let rng = Rc::new(RefCell::new(thread_rng()));

        let mut running = false;
        // Main loop
        while let Some(e) = events.next(&mut self.0.window) {
            if let Some(args) = e.press_args() {
                match args {
                    Button::Keyboard(key) => {
                        if key.eq(&Key::Space) {
                            running = !running;
                        }
                    }
                    _ => {}
                }
                self.key_press(args);
            }

            if let Some(args) = e.render_args() {
                self.render(args, &mut d_img, &mut texture);
            }

            if let Some(_args) = e.update_args() {
                if running {
                    let mut ppu = self.0.ppu.as_ref().borrow_mut();
                    let mut cpu = self.0.cpu.as_ref().borrow_mut();

                    let val = rng.as_ref().borrow_mut().gen_range(1, 16);
                    cpu.write(0xfe, val);

                    let _ = advance(&mut ppu, &mut cpu);
                }
            }
        }
    }
}