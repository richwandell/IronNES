use std::cell::RefCell;
use std::rc::Rc;
pub use crate::bus::{advance, Bus};
pub use crate::cpu::cpu_6502::Cpu;
pub use crate::ppu::Ppu;
use crate::state::State;

pub mod display;
mod cpu;
mod ppu;
pub mod cartridge;
mod bus;
mod state;
mod mapper;

pub const COLOR_BLUE: [u8; 4] = [0, 0, 255, 255];
pub const COLOR_WHITE: [u8; 4] = [255, 255, 255, 255];
pub const COLOR_GREEN: [u8; 4] = [0, 255, 0, 255];
pub const COLOR_RED: [u8; 4] = [255, 0, 0, 255];
pub const COLOR_BLACK: [u8; 4] = [0, 0, 0, 255];
pub const COLOR_GRAY: [u8; 4] = [128, 128, 128, 255];
pub const COLOR_MAGENTA: [u8; 4] = [255, 0, 255, 255];
pub const COLOR_YELLOW: [u8; 4] = [255, 255, 0, 255];
pub const COLOR_CYAN: [u8; 4] = [0, 255, 255, 255];


pub fn create_system() -> (Rc<RefCell<Bus>>, Rc<RefCell<Cpu>>, Rc<RefCell<Ppu>>, Rc<RefCell<State>>) {
    let ppu_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::ppu::Ppu::new()));
    let bus_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::bus::Bus::new()));
    let cpu_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::cpu::cpu_6502::Cpu::new()));
    let state_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::state::State::new()));

    bus_ref.clone().as_ref().borrow_mut().ppu = Some(ppu_ref.clone());
    bus_ref.clone().as_ref().borrow_mut().cpu = Some(cpu_ref.clone());
    cpu_ref.clone().as_ref().borrow_mut().state = Some(state_ref.clone());
    ppu_ref.clone().as_ref().borrow_mut().state = Some(state_ref.clone());

    (bus_ref, cpu_ref, ppu_ref, state_ref)
}
