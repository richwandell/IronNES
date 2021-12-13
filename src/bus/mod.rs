pub struct Bus {
    pub(crate) ram: Vec<u8>
}

impl Bus {

    pub fn new() -> Bus {
        Bus {
            ram: vec![0; 64 * 1024]
        }
    }

    pub(crate) fn load(&mut self, code: Vec<u8>) {
        let mut i = 0;
        for item in code {
            self.write(i, item);
            i += 1;
        }
    }

    pub(crate) fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }

    pub(crate) fn read(&mut self, addr: u16, _read_only: bool) -> u8 {
        return self.ram[addr as usize];
    }
}