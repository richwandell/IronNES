use std::cell::RefCell;
use std::rc::Rc;
use crate::bus::Bus;
use crate::state::State;

pub struct Ppu {
    pub(crate) state: Option<Rc<RefCell<State>>>,
}

impl Ppu {

    pub fn new() -> Ppu {
        Ppu {
            state: None
        }
    }

    pub fn clock(&mut self) {
        // self.state.as_ref().expect("State not ready").as_ref().borrow_mut().n_system_clock_counter += 1;
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
