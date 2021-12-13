use crate::ppu::Ppu;

pub struct Bus {
    pub(crate) ppu: Ppu,
    pub(crate) cpu_ram: Vec<u8>
}

impl Bus {

    pub fn new(ppu: Ppu) -> Bus {
        Bus {
            ppu,
            cpu_ram: vec![0; 2048]
        }
    }

    pub(crate) fn load(&mut self, code: Vec<u8>) {
        let mut i = 0;
        for item in code {
            self.cpu_write(i, item);
            i += 1;
        }
    }

    pub(crate) fn cpu_write(&mut self, addr: u16, data: u8) {
        if  addr >= 0x0000 && addr <= 0x1FFF {
            self.cpu_ram[addr as usize & 0x07ff] = data;
        } else if addr >= 0x2000 && addr <= 0x3fff {
            self.ppu.cpu_write(addr & 0x0007, data);
        }
    }

    pub(crate) fn cpu_read(&mut self, addr: u16, read_only: bool) -> u8 {
        let mut data: u8 = 0x00;
        if addr >= 0x0000 && addr <= 0x1FFF {
            data = self.cpu_ram[addr as usize & 0x07ff];
        } else if addr >= 0x2000 && addr <= 0x3fff {
            data = self.ppu.cpu_read(addr & 0x0007, read_only);
        }

        return data;
    }
}