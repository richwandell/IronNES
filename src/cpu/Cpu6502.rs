use crate::bus::Bus;
use crate::cpu::FLags::U;

struct Cpu {
    // Linkage to the communications bus
    bus: Bus,
    // All used memory addresses end up in here
    addr_abs: u16,
    // Represents absolute address following a branch
    addr_rel: u16,
    // Is the instruction byte
    opcode: u8,
    // Counts how many cycles the instruction has remaining
    cycles: u8,
    // A global accumulation of the number of clocks
    clock_count: u32,
    // Program Counter
    pc: u16,
    // Accumulator Register
    a: u8,
    // X Register
    x: u8,
    // Y Register
    y: u8,
    // Stack Pointer (points to location on bus)
    stkp: u8,
    // Status Register
    status: u8,
    // Represents the working input value to the ALU
    fetched: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            bus: Bus::new(),
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0,
            clock_count: 0,
            pc: 0x0000,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            status: 0x00,
            fetched: 0x00,
        }
    }

    fn read(&mut self, a: u16) -> u8 {
        return self.bus.read(a, false);
    }

    fn write(&mut self, a: u16, d: u8) {
        self.bus.write(a, d);
    }

    fn reset(&mut self) {
        self.addr_abs = 0xFFFC;
        let lo = self.read(self.addr_abs + 0) as u16;
        let hi = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        // Reset internal registers
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | U as u8;

        // Clear internal helper variables
        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        // Reset takes time
        self.cycles = 8;
    }
}