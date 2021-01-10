use crate::bus::Bus;
use crate::cpu::Flags::{U, I, B};
use crate::cpu::Flags;

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

    fn getFlag(&mut self, f: Flags) -> bool {
        return if (self.status & f as u8) > 0 {
            true
        } else {
            false
        }
    }

    fn setFlag(&mut self, f: Flags, v: bool) {
        if v {
            self.status |= (f as u8);
        } else {
            self.status &= !(f as u8);
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

    fn irq(&mut self) {
        if self.getFlag(I) == false {
            self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.stkp -= 1;
            self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;
            self.setFlag(B, false);
            self.setFlag(U, true);
            self.setFlag(I, true);
            self.write(0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;
            // Read new program counter location from fixed address
            self.addr_abs = 0xFFFE;
            let lo = self.read(self.addr_abs + 0);
            let hi = self.read(self.addr_abs + 1);
            self.pc = ((hi << 8) | lo) as u16;

            // IRQs take time
            self.cycles = 7;
        }
    }

    fn nmi(&mut self) {
        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;
        self.setFlag(B, false);
        self.setFlag(U, true);
        self.setFlag(I, true);

        self.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.addr_abs = 0xFFFA;
        let lo = self.read(self.addr_abs + 0);
        let hi = self.read(self.addr_abs + 1);

        self.pc = ((hi << 8) | lo) as u16;

        self.cycles = 8;
    }

    fn imp(&mut self) -> bool {
        self.fetched = self.a;
        false
    }

    fn imm(&mut self) -> bool {
        self.pc += 1;
        self.addr_abs = self.pc;
        false
    }

    fn zp0(&mut self) -> bool {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    fn zpx(&mut self) -> bool {
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    fn zpy(&mut self) -> bool {
        self.addr_abs = (self.read(self.pc) + self.y) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }
}