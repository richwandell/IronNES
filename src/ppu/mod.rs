use std::cell::RefCell;
use std::rc::Rc;
use crate::bus::Bus;
use crate::state::State;
use graphics::{Image};
use image::Rgba;
use rand::Rng;
use crate::display::{EMU_HEIGHT, EMU_WIDTH};

pub struct Ppu {
    pub(crate) state: Option<Rc<RefCell<State>>>,
    pub pallet: Vec<Rgba<u8>>,
    pub screen: Vec<Vec<Rgba<u8>>>,
    pub cycle: u32,
    pub scan_line: u32,
    pub frame_complete: bool,
}

impl Ppu {

    pub fn new() -> Ppu {
        let mut screen: Vec<Vec<Rgba<u8>>> = vec![];

        for y in 0..EMU_HEIGHT {
            let mut row = vec![];
            for x in 0..EMU_WIDTH {
                row.push(image::Rgba([0, 0, 0, 255]));
            }
            screen.push(row);
        }

        Ppu {
            frame_complete: false,
            cycle: 0,
            scan_line: 0,
            screen,
            pallet: create_pallet(),
            state: None
        }
    }

    pub fn clock(&mut self) {
        let mut rng = rand::thread_rng();

        let rnd: usize = rng.gen();
        let pixel = if rnd % 2 == 0 {
            0x3F
        } else {
            0x30
        };

        let x = (self.cycle) as usize;
        let y = self.scan_line as usize;
        self.state
            .as_ref()
            .expect("Missing State")
            .as_ref()
            .borrow_mut()
            .screen[y][x] = self.pallet[pixel];
        self.cycle += 1;
        if self.cycle == EMU_WIDTH - 1 {
            self.cycle = 0;
            self.scan_line += 1;
            if self.scan_line == EMU_HEIGHT - 1 {
                self.scan_line = 0;
                self.frame_complete = true;
            }
        }
        // Fake some noise for now
        // sprScreen.SetPixel(cycle - 1, scanline, palScreen[(rand() % 2) ? 0x3F : 0x30]);
        //
        // // Advance renderer - it never stops, it's relentless
        // cycle++;
        // if (cycle >= 341)
        // {
        //     cycle = 0;
        //     scanline++;
        //     if (scanline >= 261)
        //     {
        //         scanline = -1;
        //         frame_complete = true;
        //     }
        // }
    }
}

pub(crate) fn cpu_read(state: &mut State, addr: u16, read_only: bool) -> u8 {
    let data = 0x00;

    match addr {
        // Control
        0x0000 => {

        }
        // Mask
        0x0001 => {

        }
        // Status
        0x0002 => {

        }
        // OAM Address
        0x0003 => {

        }
        // OAM Data
        0x0004 => {

        }
        // Scroll
        0x0005 => {

        }
        // PPU Address
        0x0006 => {

        }
        // PPU Data
        0x0007 => {

        }
        _ => {}
    }

    data
}

pub(crate) fn cpu_write(state: &mut State, addr: u16, data: u8) {
    match addr {
        // Control
        0x0000 => {

        }
        // Mask
        0x0001 => {

        }
        // Status
        0x0002 => {

        }
        // OAM Address
        0x0003 => {

        }
        // OAM Data
        0x0004 => {

        }
        // Scroll
        0x0005 => {

        }
        // PPU Address
        0x0006 => {

        }
        // PPU Data
        0x0007 => {

        }
        _ => {}
    }
}

fn create_pallet() -> Vec<Rgba<u8>> {
    vec![
        image::Rgba([84, 84, 84, 255]),
        image::Rgba([0, 30, 116, 255]),
        image::Rgba([8, 16, 144, 255]),
        image::Rgba([48, 0, 136, 255]),
        image::Rgba([68, 0, 100, 255]),
        image::Rgba([92, 0, 48, 255]),
        image::Rgba([84, 4, 0, 255]),
        image::Rgba([60, 24, 0, 255]),
        image::Rgba([32, 42, 0, 255]),
        image::Rgba([8, 58, 0, 255]),
        image::Rgba([0, 64, 0, 255]),
        image::Rgba([0, 60, 0, 255]),
        image::Rgba([0, 50, 60, 255]),
        image::Rgba([0, 0, 0, 255]),
        image::Rgba([0, 0, 0, 255]),
        image::Rgba([0, 0, 0, 255]),
    
        image::Rgba([152, 150, 152, 255]),
        image::Rgba([8, 76, 196, 255]),
        image::Rgba([48, 50, 236, 255]),
        image::Rgba([92, 30, 228, 255]),
        image::Rgba([136, 20, 176, 255]),
        image::Rgba([160, 20, 100, 255]),
        image::Rgba([152, 34, 32, 255]),
        image::Rgba([120, 60, 0, 255]),
        image::Rgba([84, 90, 0, 255]),
        image::Rgba([40, 114, 0, 255]),
        image::Rgba([8, 124, 0, 255]),
        image::Rgba([0, 118, 40, 255]),
        image::Rgba([0, 102, 120, 255]),
        image::Rgba([0, 0, 0, 255]),
        image::Rgba([0, 0, 0, 255]),
        image::Rgba([0, 0, 0, 255]),
    
        image::Rgba([236, 238, 236, 255]),
        image::Rgba([76, 154, 236, 255]),
        image::Rgba([120, 124, 236, 255]),
        image::Rgba([176, 98, 236, 255]),
        image::Rgba([228, 84, 236, 255]),
        image::Rgba([236, 88, 180, 255]),
        image::Rgba([236, 106, 100, 255]),
        image::Rgba([212, 136, 32, 255]),
        image::Rgba([160, 170, 0, 255]),
        image::Rgba([116, 196, 0, 255]),
        image::Rgba([76, 208, 32, 255]),
        image::Rgba([56, 204, 108, 255]),
        image::Rgba([56, 180, 204, 255]),
        image::Rgba([60, 60, 60, 255]),
        image::Rgba([0, 0, 0, 255]),
        image::Rgba([0, 0, 0, 255]),
    
        image::Rgba([236, 238, 236, 255]),
        image::Rgba([168, 204, 236, 255]),
        image::Rgba([188, 188, 236, 255]),
        image::Rgba([212, 178, 236, 255]),
        image::Rgba([236, 174, 236, 255]),
        image::Rgba([236, 174, 212, 255]),
        image::Rgba([236, 180, 176, 255]),
        image::Rgba([228, 196, 144, 255]),
        image::Rgba([204, 210, 120, 255]),
        image::Rgba([180, 222, 120, 255]),
        image::Rgba([168, 226, 144, 255]),
        image::Rgba([152, 226, 180, 255]),
        image::Rgba([160, 214, 228, 255]),
        image::Rgba([160, 162, 160, 255]),
        image::Rgba([0, 0, 0, 255]),
        image::Rgba([0, 0, 0, 255])
    ]
}