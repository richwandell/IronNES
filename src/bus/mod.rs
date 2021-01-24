pub struct Bus {
    ram: Vec<u8>
}

impl Bus {

    pub fn new() -> Bus {
        Bus {
            ram: vec![0; 64 * 1024]
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if  addr >= 0x0000 && addr <= 0xFFFF {
            self.ram[addr as usize] = data;
        }
    }

    pub fn read(&mut self, addr: u16, _read_only: bool) -> u8 {
        if addr >= 0x0000 && addr <= 0xFFFF {
            return self.ram[addr as usize];
        }

        return 0x00;
    }
}