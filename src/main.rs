use std::cell::RefCell;
use std::rc::Rc;
use crate::bus::{advance, Bus};
use crate::cpu::cpu_6502::Cpu;
use crate::display::display::Display;
use crate::ppu::Ppu;
use crate::state::State;

mod display;
mod cpu;
mod ppu;
mod cartridge;
mod bus;
mod state;

fn create_system() -> (Rc<RefCell<Bus>>, Rc<RefCell<Cpu>>, Rc<RefCell<Ppu>>, Rc<RefCell<State>>) {
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

fn main() {

    let (_bus_ref, cpu_ref, ppu_ref, state_ref) = create_system();

    // let code: Vec<u8> = vec![
    //     0xA2, 0x0A, 0x86, 0x00, 0x86, 0x01, 0xE6, 0x00,
    //     0xC6, 0x01, 0xA6, 0x00, 0x8E, 0x00, 0x03, 0xA6,
    //     0x01, 0x8E, 0x01, 0x03
    // ];

    let code : Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC,
        0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];

    state_ref.as_ref().borrow_mut().load(code, 0x8000);
    cpu_ref.as_ref().borrow_mut().reset();

    let mut display = Display::new(state_ref.clone(), cpu_ref.clone());

    display.start(|event| {
        let mut ppu= ppu_ref.as_ref().borrow_mut();
        let mut cpu = cpu_ref.as_ref().borrow_mut();

        if let Some(_args) = event {
            if let Ok(_) = advance(&mut ppu, &mut cpu) {
                println!("{}", "clock ok");
            } else {
                println!("{}", "clock not ok")
            }
        }
    });
}