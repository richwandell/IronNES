use crate::bus::Bus;
use crate::cpu::Flags::{U, I, B, C, Z, V, N};
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
                Instruction{ name: "Brk".to_string(), operate: Opcodes::Brk, addr: AddressModes::Imm, cycles: 7}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Zp0, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Php".to_string(), operate: Opcodes::Php, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6},
                Instruction{ name: "Bpl".to_string(), operate: Opcodes::Bpl, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Zpx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Clc".to_string(), operate: Opcodes::Clc, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Abx, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7},
                Instruction{ name: "Jsr".to_string(), operate: Opcodes::Jsr, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "Bit".to_string(), operate: Opcodes::Bit, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Zp0, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Plp".to_string(), operate: Opcodes::Plp, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Bit".to_string(), operate: Opcodes::Bit, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6},
                Instruction{ name: "Bmi".to_string(), operate: Opcodes::Bmi, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Zpx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Sec".to_string(), operate: Opcodes::Sec, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Abx, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7},
                Instruction{ name: "Rti".to_string(), operate: Opcodes::Rti, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Zp0, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Pha".to_string(), operate: Opcodes::Pha, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Jmp".to_string(), operate: Opcodes::Jmp, addr: AddressModes::Abs, cycles: 3}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6},
                Instruction{ name: "Bvc".to_string(), operate: Opcodes::Bvc, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Zpx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Cli".to_string(), operate: Opcodes::Cli, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Abx, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7},
                Instruction{ name: "Rts".to_string(), operate: Opcodes::Rts, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Zp0, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Pla".to_string(), operate: Opcodes::Pla, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Jmp".to_string(), operate: Opcodes::Jmp, addr: AddressModes::Ind, cycles: 5}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6},
                Instruction{ name: "Bvs".to_string(), operate: Opcodes::Bvs, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Zpx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Sei".to_string(), operate: Opcodes::Sei, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Abx, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7},
                Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Sty".to_string(), operate: Opcodes::Sty, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Stx".to_string(), operate: Opcodes::Stx, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Dey".to_string(), operate: Opcodes::Dey, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Txa".to_string(), operate: Opcodes::Txa, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Sty".to_string(), operate: Opcodes::Sty, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Stx".to_string(), operate: Opcodes::Stx, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4},
                Instruction{ name: "Bcc".to_string(), operate: Opcodes::Bcc, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Izy, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Sty".to_string(), operate: Opcodes::Sty, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Stx".to_string(), operate: Opcodes::Stx, addr: AddressModes::Zpy, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Tya".to_string(), operate: Opcodes::Tya, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Aby, cycles: 5}, Instruction{ name: "Txs".to_string(), operate: Opcodes::Txs, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Abx, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5},
                Instruction{ name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 3}, Instruction{ name: "Tay".to_string(), operate: Opcodes::Tay, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Tax".to_string(), operate: Opcodes::Tax, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4},
                Instruction{ name: "Bcs".to_string(), operate: Opcodes::Bcs, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Zpy, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Clv".to_string(), operate: Opcodes::Clv, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "Tsx".to_string(), operate: Opcodes::Tsx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4},
                Instruction{ name: "Cpy".to_string(), operate: Opcodes::Cpy, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "Cpy".to_string(), operate: Opcodes::Cpy, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Zp0, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Iny".to_string(), operate: Opcodes::Iny, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Dex".to_string(), operate: Opcodes::Dex, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Cpy".to_string(), operate: Opcodes::Cpy, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6},
                Instruction{ name: "Bne".to_string(), operate: Opcodes::Bne, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Zpx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Cld".to_string(), operate: Opcodes::Cld, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "Nop".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Abx, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7},
                Instruction{ name: "Cpx".to_string(), operate: Opcodes::Cpx, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Izx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "Cpx".to_string(), operate: Opcodes::Cpx, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Zp0, cycles: 3}, Instruction{ name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Zp0, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5}, Instruction{ name: "Inx".to_string(), operate: Opcodes::Inx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Imm, cycles: 2}, Instruction{ name: "Nop".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Cpx".to_string(), operate: Opcodes::Cpx, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Abs, cycles: 4}, Instruction{ name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Abs, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6},
                Instruction{ name: "Beq".to_string(), operate: Opcodes::Beq, addr: AddressModes::Rel, cycles: 2}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Izy, cycles: 5}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Zpx, cycles: 4}, Instruction{ name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Zpx, cycles: 6}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6}, Instruction{ name: "Sed".to_string(), operate: Opcodes::Sed, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Aby, cycles: 4}, Instruction{ name: "Nop".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 4}, Instruction{ name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Abx, cycles: 4}, Instruction{ name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Abx, cycles: 7}, Instruction{ name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7},
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
            self.status |= f as u8;
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
        if self.lookup[self.opcode as usize].addr != AddressModes::Imp {
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

    fn adc(&mut self) -> bool {
        self.fetch();

        let temp = self.a as u16 + self.fetched as u16 + self.getFlag(C) as u16;

        self.setFlag(C, temp > 255);

        self.setFlag(Z, (temp & 0x00FF) == 0);

        self.setFlag(V, (!(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ temp)) & 0x0080 > 0);

        self.setFlag(N, temp & 0x80 > 0);

        self.a = (temp & 0x00FF) as u8;

        return true;
    }

    fn sbc(&mut self) -> bool {
        self.fetch();

        let value = self.fetched as u16 ^ 0x00FF;

        let temp = self.a as u16 + value + self.getFlag(C) as u16;

        self.setFlag(C, (temp & 0xFF00) > 0);

        self.setFlag(Z, (temp & 0x00FF) == 0);

        self.setFlag(V, ((temp ^ self.a as u16) & (temp ^ value) & 0x0080) > 0);

        self.setFlag(N, temp & 0x0080 > 0);

        self.a = (temp & 0x00FF) as u8;

        return true;
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