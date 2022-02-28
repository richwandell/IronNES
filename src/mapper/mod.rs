pub mod mapper0;

use crate::State;

pub trait Mapper {
    fn cpu_map_read(&mut self, state: &State, addr: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_map_write(&mut self, state: &State, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_read(&mut self, state: State, addr: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_write(&mut self, state: State, addr: u16, mapped_addr: &mut u32) -> bool;
}

