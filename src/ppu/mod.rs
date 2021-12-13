use crate::bus::Bus;

pub struct Ppu {

}

impl Ppu {

    pub fn new() -> Ppu {
        Ppu {

        }
    }

    pub(crate) fn cpu_write(&mut self, addr: u16, data: u8) {

    }

    pub(crate) fn cpu_read(&mut self, addr: u16, _read_only: bool) -> u8 {
        return 0x00;
    }

    pub(crate) fn ppu_write(&mut self, addr: u16, data: u8) {
    }

    pub(crate) fn ppu_read(&mut self, addr: u16, _read_only: bool) -> u8 {
        return 0x00;
    }
}