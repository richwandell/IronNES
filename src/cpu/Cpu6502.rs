use crate::bus::Bus;
use crate::cpu::Flags::{U, I, B};
use crate::cpu::{Flags, Opcodes, AddressModes};

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
    lookup: Vec<Instruction>
}

struct Instruction {
    name: String,
    operate: Opcodes,
    addr: AddressModes,
    cycles: usize
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
            lookup: vec![
                Instruction{ name: "brk".to_string(), operate: Opcodes::brk, addr: AddressModes::imm, cycles: 7},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 3},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "asl".to_string(), operate: Opcodes::asl, addr: AddressModes::zp0, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "php".to_string(), operate: Opcodes::php, addr: AddressModes::imp, cycles: 3},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::imm, cycles: 2},Instruction{ name: "asl".to_string(), operate: Opcodes::asl, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::abs, cycles: 4},Instruction{ name: "asl".to_string(), operate: Opcodes::asl, addr: AddressModes::abs, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},
                Instruction{ name: "bpl".to_string(), operate: Opcodes::bpl, addr: AddressModes::rel, cycles: 2},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "asl".to_string(), operate: Opcodes::asl, addr: AddressModes::zpx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "clc".to_string(), operate: Opcodes::clc, addr: AddressModes::imp, cycles: 2},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::aby, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "ora".to_string(), operate: Opcodes::ora, addr: AddressModes::abx, cycles: 4},Instruction{ name: "asl".to_string(), operate: Opcodes::asl, addr: AddressModes::abx, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},
                Instruction{ name: "jsr".to_string(), operate: Opcodes::jsr, addr: AddressModes::abs, cycles: 6},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "bit".to_string(), operate: Opcodes::bit, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "rol".to_string(), operate: Opcodes::rol, addr: AddressModes::zp0, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "plp".to_string(), operate: Opcodes::plp, addr: AddressModes::imp, cycles: 4},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::imm, cycles: 2},Instruction{ name: "rol".to_string(), operate: Opcodes::rol, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "bit".to_string(), operate: Opcodes::bit, addr: AddressModes::abs, cycles: 4},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::abs, cycles: 4},Instruction{ name: "rol".to_string(), operate: Opcodes::rol, addr: AddressModes::abs, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},
                Instruction{ name: "bmi".to_string(), operate: Opcodes::bmi, addr: AddressModes::rel, cycles: 2},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "rol".to_string(), operate: Opcodes::rol, addr: AddressModes::zpx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "sec".to_string(), operate: Opcodes::sec, addr: AddressModes::imp, cycles: 2},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::aby, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "and".to_string(), operate: Opcodes::and, addr: AddressModes::abx, cycles: 4},Instruction{ name: "rol".to_string(), operate: Opcodes::rol, addr: AddressModes::abx, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},
                Instruction{ name: "rti".to_string(), operate: Opcodes::rti, addr: AddressModes::imp, cycles: 6},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 3},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "lsr".to_string(), operate: Opcodes::lsr, addr: AddressModes::zp0, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "pha".to_string(), operate: Opcodes::pha, addr: AddressModes::imp, cycles: 3},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::imm, cycles: 2},Instruction{ name: "lsr".to_string(), operate: Opcodes::lsr, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "jmp".to_string(), operate: Opcodes::jmp, addr: AddressModes::abs, cycles: 3},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::abs, cycles: 4},Instruction{ name: "lsr".to_string(), operate: Opcodes::lsr, addr: AddressModes::abs, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},
                Instruction{ name: "bvc".to_string(), operate: Opcodes::bvc, addr: AddressModes::rel, cycles: 2},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "lsr".to_string(), operate: Opcodes::lsr, addr: AddressModes::zpx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "cli".to_string(), operate: Opcodes::cli, addr: AddressModes::imp, cycles: 2},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::aby, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "eor".to_string(), operate: Opcodes::eor, addr: AddressModes::abx, cycles: 4},Instruction{ name: "lsr".to_string(), operate: Opcodes::lsr, addr: AddressModes::abx, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},
                Instruction{ name: "rts".to_string(), operate: Opcodes::rts, addr: AddressModes::imp, cycles: 6},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 3},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "ror".to_string(), operate: Opcodes::ror, addr: AddressModes::zp0, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "pla".to_string(), operate: Opcodes::pla, addr: AddressModes::imp, cycles: 4},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::imm, cycles: 2},Instruction{ name: "ror".to_string(), operate: Opcodes::ror, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "jmp".to_string(), operate: Opcodes::jmp, addr: AddressModes::ind, cycles: 5},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::abs, cycles: 4},Instruction{ name: "ror".to_string(), operate: Opcodes::ror, addr: AddressModes::abs, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},
                Instruction{ name: "bvs".to_string(), operate: Opcodes::bvs, addr: AddressModes::rel, cycles: 2},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "ror".to_string(), operate: Opcodes::ror, addr: AddressModes::zpx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "sei".to_string(), operate: Opcodes::sei, addr: AddressModes::imp, cycles: 2},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::aby, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "adc".to_string(), operate: Opcodes::adc, addr: AddressModes::abx, cycles: 4},Instruction{ name: "ror".to_string(), operate: Opcodes::ror, addr: AddressModes::abx, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},
                Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "sty".to_string(), operate: Opcodes::sty, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "stx".to_string(), operate: Opcodes::stx, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 3},Instruction{ name: "dey".to_string(), operate: Opcodes::dey, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "txa".to_string(), operate: Opcodes::txa, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "sty".to_string(), operate: Opcodes::sty, addr: AddressModes::abs, cycles: 4},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::abs, cycles: 4},Instruction{ name: "stx".to_string(), operate: Opcodes::stx, addr: AddressModes::abs, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 4},
                Instruction{ name: "bcc".to_string(), operate: Opcodes::bcc, addr: AddressModes::rel, cycles: 2},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::izy, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "sty".to_string(), operate: Opcodes::sty, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "stx".to_string(), operate: Opcodes::stx, addr: AddressModes::zpy, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 4},Instruction{ name: "tya".to_string(), operate: Opcodes::tya, addr: AddressModes::imp, cycles: 2},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::aby, cycles: 5},Instruction{ name: "txs".to_string(), operate: Opcodes::txs, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 5},Instruction{ name: "sta".to_string(), operate: Opcodes::sta, addr: AddressModes::abx, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},
                Instruction{ name: "ldy".to_string(), operate: Opcodes::ldy, addr: AddressModes::imm, cycles: 2},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::izx, cycles: 6},Instruction{ name: "ldx".to_string(), operate: Opcodes::ldx, addr: AddressModes::imm, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "ldy".to_string(), operate: Opcodes::ldy, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "ldx".to_string(), operate: Opcodes::ldx, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 3},Instruction{ name: "tay".to_string(), operate: Opcodes::tay, addr: AddressModes::imp, cycles: 2},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::imm, cycles: 2},Instruction{ name: "tax".to_string(), operate: Opcodes::tax, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "ldy".to_string(), operate: Opcodes::ldy, addr: AddressModes::abs, cycles: 4},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::abs, cycles: 4},Instruction{ name: "ldx".to_string(), operate: Opcodes::ldx, addr: AddressModes::abs, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 4},
                Instruction{ name: "bcs".to_string(), operate: Opcodes::bcs, addr: AddressModes::rel, cycles: 2},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "ldy".to_string(), operate: Opcodes::ldy, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "ldx".to_string(), operate: Opcodes::ldx, addr: AddressModes::zpy, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 4},Instruction{ name: "clv".to_string(), operate: Opcodes::clv, addr: AddressModes::imp, cycles: 2},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::aby, cycles: 4},Instruction{ name: "tsx".to_string(), operate: Opcodes::tsx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 4},Instruction{ name: "ldy".to_string(), operate: Opcodes::ldy, addr: AddressModes::abx, cycles: 4},Instruction{ name: "lda".to_string(), operate: Opcodes::lda, addr: AddressModes::abx, cycles: 4},Instruction{ name: "ldx".to_string(), operate: Opcodes::ldx, addr: AddressModes::aby, cycles: 4},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 4},
                Instruction{ name: "cpy".to_string(), operate: Opcodes::cpy, addr: AddressModes::imm, cycles: 2},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "cpy".to_string(), operate: Opcodes::cpy, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "dec".to_string(), operate: Opcodes::dec, addr: AddressModes::zp0, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "iny".to_string(), operate: Opcodes::iny, addr: AddressModes::imp, cycles: 2},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::imm, cycles: 2},Instruction{ name: "dex".to_string(), operate: Opcodes::dex, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "cpy".to_string(), operate: Opcodes::cpy, addr: AddressModes::abs, cycles: 4},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::abs, cycles: 4},Instruction{ name: "dec".to_string(), operate: Opcodes::dec, addr: AddressModes::abs, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},
                Instruction{ name: "bne".to_string(), operate: Opcodes::bne, addr: AddressModes::rel, cycles: 2},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "dec".to_string(), operate: Opcodes::dec, addr: AddressModes::zpx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "cld".to_string(), operate: Opcodes::cld, addr: AddressModes::imp, cycles: 2},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::aby, cycles: 4},Instruction{ name: "nop".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "cmp".to_string(), operate: Opcodes::cmp, addr: AddressModes::abx, cycles: 4},Instruction{ name: "dec".to_string(), operate: Opcodes::dec, addr: AddressModes::abx, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},
                Instruction{ name: "cpx".to_string(), operate: Opcodes::cpx, addr: AddressModes::imm, cycles: 2},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::izx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "cpx".to_string(), operate: Opcodes::cpx, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::zp0, cycles: 3},Instruction{ name: "inc".to_string(), operate: Opcodes::inc, addr: AddressModes::zp0, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 5},Instruction{ name: "inx".to_string(), operate: Opcodes::inx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::imm, cycles: 2},Instruction{ name: "nop".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::sbc, addr: AddressModes::imp, cycles: 2},Instruction{ name: "cpx".to_string(), operate: Opcodes::cpx, addr: AddressModes::abs, cycles: 4},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::abs, cycles: 4},Instruction{ name: "inc".to_string(), operate: Opcodes::inc, addr: AddressModes::abs, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},
                Instruction{ name: "beq".to_string(), operate: Opcodes::beq, addr: AddressModes::rel, cycles: 2},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::izy, cycles: 5},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 8},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::zpx, cycles: 4},Instruction{ name: "inc".to_string(), operate: Opcodes::inc, addr: AddressModes::zpx, cycles: 6},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 6},Instruction{ name: "sed".to_string(), operate: Opcodes::sed, addr: AddressModes::imp, cycles: 2},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::aby, cycles: 4},Instruction{ name: "nop".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 2},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::nop, addr: AddressModes::imp, cycles: 4},Instruction{ name: "sbc".to_string(), operate: Opcodes::sbc, addr: AddressModes::abx, cycles: 4},Instruction{ name: "inc".to_string(), operate: Opcodes::inc, addr: AddressModes::abx, cycles: 7},Instruction{ name: "???".to_string(), operate: Opcodes::xxx, addr: AddressModes::imp, cycles: 7},
            ]
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

    fn fetch(&mut self) -> u8 {
        if self.lookup[self.opcode as usize].addr != AddressModes::imp {
            self.fetched = self.read(self.addr_abs);
        }
        return self.fetched;
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

    fn rel(&mut self) -> bool {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if self.addr_rel & 0x80 > 0 {
            self.addr_rel |= 0xFF00;
        }
        return false;
    }

    fn abs(&mut self) -> bool {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        return false;
    }

    fn abx(&mut self) -> bool {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return true;
        } else {
            return false;
        }
    }

    fn aby(&mut self) -> bool {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return true;
        } else {
            return false;
        }
    }

    fn ind(&mut self) -> bool {
        let ptr_lo = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            self.addr_abs = ((self.read(ptr & 0xFF00) << 8) | self.read(ptr + 0)) as u16;
        } else {
            self.addr_abs = ((self.read(ptr + 1) << 8) | self.read(ptr + 0)) as u16;
        }

        return false;
    }

    fn izx(&mut self) -> bool {
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = (self.read(t + self.x as u16) & 0x00FF) as u16;
        let hi = (self.read(t + self.x as u16 + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        return false;
    }

    fn izy(&mut self) -> bool {
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = self.read(t & 0x00FF) as u16;
        let hi = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if self.addr_abs & 0xFF00 != (hi << 8) {
            return true;
        }

        return false;
    }

    fn brk(&mut self) -> bool {
        self.pc += 1;

        self.setFlag(I, true);

        self.write((0x0100 + self.stkp as u16) as u16, ((self.pc >> 8) & 0x00FF) as u8);

        self.stkp -= 1;

        self.write((0x0100 + self.stkp as u16) as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.setFlag(B, true);

        self.write((0x0100 + self.stkp as u16) as u16, self.status);

        self.stkp -= 1;

        self.setFlag(B, false);

        self.pc = self.read(0xFFFE) as u16 | (self.read(0xFFFF) << 8) as u16;

        return false;
    }
}