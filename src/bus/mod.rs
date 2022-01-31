use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::cpu::cpu_6502::Cpu;
use crate::ppu::Ppu;
use crate::state::State;

pub(crate) fn cpu_read(state: &State, addr: u16, read_only: bool) -> u8 {
    let mut data: u8 = 0x00;
    if addr >= 0x0000 && addr <= 0xFFFF {
        data = state.cpu_ram[addr as usize];
    } else if addr >= 0x2000 && addr <= 0x3fff {
        data = state.ppu_ram[addr as usize];
    }
    // if addr >= 0x0000 && addr <= 0x1FFF {
    //     data = state.cpu_ram[addr as usize & 0x07ff];
    // } else if addr >= 0x2000 && addr <= 0x3fff {
    //     data = state.ppu_ram[addr as usize & 0x0007];
    // }
    return data;
}

pub(crate) fn cpu_write(state: &mut State, addr: u16, data: u8) {
    if  addr >= 0x0000 && addr <= 0xFFFF {
        state.cpu_ram[addr as usize] = data;
    } else if addr >= 0x2000 && addr <= 0x3fff {
        state.ppu_ram[addr as usize] = data;
    }
    // if  addr >= 0x0000 && addr <= 0x1FFF {
    //     let location = addr as usize & 0x07ff;
    //     state.cpu_ram[location] = data;
    // } else if addr >= 0x2000 && addr <= 0x3fff {
    //     let location = addr as usize & 0x0007;
    //     state.ppu_ram[location] = data;
    // }
}

pub(crate) fn reset(state: &mut State, ppu: &mut Ppu, cpu: &mut Cpu) {
    cpu.reset();
    state.n_system_clock_counter = 0;
}

pub(crate) fn clock(ppu: &mut Ppu, cpu: &mut Cpu) -> Result<(), ()>{
    ppu.clock();
    if cpu.get_state_mut().n_system_clock_counter % 3 == 0 {
        if let Err(_) = cpu.clock() {
            return Err(());
        }
    }
    cpu.get_state_mut().n_system_clock_counter += 1;
    return Ok(());
}

pub(crate) fn advance(ppu: &mut Ppu, cpu: &mut Cpu) -> Result<(), ()>{
    loop {
        ppu.clock();
        if cpu.get_state_mut().n_system_clock_counter % 3 == 0 {
            if let Err(_) = cpu.clock() {
                return Err(());
            } else if cpu.cycles == 0 {
                return Ok(())
            }
        }
        cpu.get_state_mut().n_system_clock_counter += 1;

        println!("cycling");
    }
}

pub struct Bus {
    pub(crate) cpu: Option<Rc<RefCell<Cpu>>>,
    pub(crate) ppu: Option<Rc<RefCell<Ppu>>>,
    pub(crate) state: Option<Rc<RefCell<State>>>,
}

impl Bus {

    pub fn new() -> Bus {
        Bus {
            cpu: None,
            ppu: None,
            state: None
        }
    }
}