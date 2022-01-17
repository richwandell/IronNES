use crate::bus::Bus;
use crate::cpu::cpu_6502::Cpu;
use crate::ppu::Ppu;

mod flags;
mod math;

macro_rules! create_devices {
    ($var1:ident, $var2:ident) => {
        let ppu_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::ppu::Ppu::new()));
        let bus_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::bus::Bus::new()));
        let cpu_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::cpu::cpu_6502::Cpu::new()));
        let state_ref = std::rc::Rc::new(std::cell::RefCell::new(crate::state::State::new()));

        bus_ref.clone().as_ref().borrow_mut().ppu = Some(ppu_ref.clone());
        bus_ref.clone().as_ref().borrow_mut().cpu = Some(cpu_ref.clone());
        cpu_ref.clone().as_ref().borrow_mut().state = Some(state_ref.clone());
        ppu_ref.clone().as_ref().borrow_mut().state = Some(state_ref.clone());

        let mut $var1 = ppu_ref.as_ref().borrow_mut();
        let mut $var2 = cpu_ref.as_ref().borrow_mut();
    };
}

pub(crate) use create_devices;


