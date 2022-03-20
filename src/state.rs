use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::bus::mem_write;
use crate::cartridge::Cartridge;
use crate::mapper::Mapper;
use crate::mapper::mapper0::Mapper0;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};
use image::{ImageBuffer, Rgba};
use opengl_graphics::{Texture, TextureSettings};

pub struct State {
    pub(crate) cpu_ram: Vec<u8>,
    pub(crate) ppu_ram: Vec<u8>,
    pub(crate) code_end: usize,
    pub(crate) ppu_name_tables: Vec<Vec<u8>>,
    pub(crate) ppu_palette_table: Vec<u8>,
    pub(crate) n_system_clock_counter: usize,
    pub cartridge: Option<Rc<RefCell<Cartridge>>>,
    pub mapper: usize,
    pub screen: Vec<Vec<Rgba<u8>>>,
}

impl State {

    pub fn new() -> State {
        let mut screen: Vec<Vec<Rgba<u8>>> = vec![];

        for _ in 0..EMU_HEIGHT {
            let mut row = vec![];
            for _ in 0..EMU_WIDTH {
                row.push(image::Rgba([0, 0, 0, 255]));
            }
            screen.push(row);
        }

        State {
            screen,
            cpu_ram: vec![0; 64 * 1024],
            ppu_ram: vec![0; 2048],
            code_end: 0,
            ppu_name_tables: vec![vec![0; 1024], vec![0; 1024]],
            ppu_palette_table: vec![0; 32],
            n_system_clock_counter: 0,
            cartridge: None,
            mapper: 0
        }
    }

    pub fn get_cartridge(&self) -> Ref<'_, Cartridge> {
        self.cartridge.as_ref().expect("Missing cart").as_ref().borrow()
    }

    pub fn connect_cartridge(&mut self, cartridge: Option<Rc<RefCell<Cartridge>>>) {
        self.cartridge = cartridge;
    }

    pub fn get_mapper(&self) -> impl Mapper {
        let cart = self.cartridge.as_ref().expect("Missing cart").as_ref().borrow();
        if cart.n_mapper_id == 0 {
            Mapper0 {}
        } else {
            Mapper0 {}
        }
    }

    pub fn load(&mut self, code: Vec<u8>, offset: u16) {
        let end = code.len().clone();
        self.code_end = end + offset as usize;

        let mut i = 0;
        for item in code {
            mem_write(self, i + offset, item);
            i += 1;
        }

        let offset_bytes = offset.to_be_bytes();
        mem_write(self, 0xFFFC, offset_bytes[1]);
        mem_write(self, 0xFFFD, offset_bytes[0]);
    }
}