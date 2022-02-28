use crate::mapper::Mapper;
use crate::State;

pub struct Mapper0;

impl Mapper for Mapper0 {

    fn cpu_map_read(&mut self, state: &State, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 && addr <= 0xffff {
            let cart = state.cartridge.as_ref().expect("Missing cart").as_ref().borrow_mut();
            let banks = cart.n_prgbanks;
            let mapped = (addr & if banks > 1 {
                0x7fff
            } else {
                0x3fff
            }) as u32;

            *mapped_addr = mapped;
            return true;
        }
        return false;
    }

    fn cpu_map_write(&mut self, state: State, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x8000 && addr <= 0xffff {
            let cart = state.cartridge.as_ref().expect("Missing cart").as_ref().borrow_mut();
            let banks = cart.n_prgbanks;
            let mapped = (addr & if banks > 1 {
                0x7fff
            } else {
                0x3fff
            }) as u32;

            *mapped_addr = mapped;
            return true;
        }
        return false;
    }

    fn ppu_map_read(&mut self, state: State, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x0000 && addr <= 0x1fff {
            *mapped_addr = addr as u32;
            return true;
        }
        return false;
    }

    fn ppu_map_write(&mut self, state: State, addr: u16, mapped_addr: &mut u32) -> bool {
        if addr >= 0x0000 && addr <= 0x1FFF {
            let cart = state.cartridge.as_ref().expect("Missing cart").as_ref().borrow_mut();
            let banks = cart.n_prgbanks;

            if banks == 0 {
                // Treat as RAM
                *mapped_addr = addr as u32;
                return true;
            }
        }

        return false;
    }
}