use crate::bus::Bus;
use crate::cpu::cpu_6502::Cpu;
use crate::ppu::Ppu;

mod flags;
mod math;

pub(crate) fn create_cpu() -> Cpu {
    return Cpu::new(Bus::new(Ppu::new()))
}

