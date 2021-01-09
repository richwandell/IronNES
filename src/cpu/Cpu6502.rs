use crate::bus::Bus;

struct Cpu {
    bus: Bus
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            bus: Bus::new()
        }
    }

    fn read(&mut self, a: u16) -> u8 {
        return self.bus.read(a, false);
    }

    fn write(&mut self, a: u16, d: u8) {
        self.bus.write(a, d);
    }
}