use std::cell::RefCell;
use std::rc::Rc;
pub use crate::bus::{advance, Bus};
pub use crate::cpu::cpu_6502::Cpu;
pub use crate::display::display::Display;
pub use crate::ppu::Ppu;
use crate::state::State;

pub mod display;
mod cpu;
mod ppu;
mod cartridge;
mod bus;
mod state;

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
