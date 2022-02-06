use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;
use graphics::math::add;
use crate::bus::{Bus, cpu_read, cpu_write};
use crate::cpu::Flags::{U, I, B, C, Z, V, N, D};
use crate::cpu::{Flags, Opcodes, AddressModes};
use crate::cpu::AddressModes::Imp;
use crate::state::State;

pub struct Cpu {
    pub(crate) state: Option<Rc<RefCell<State>>>,
    // All used memory addresses end up in here
    addr_abs: u16,
    // Represents absolute address following a branch
    addr_rel: u16,
    // Is the instruction byte
    opcode: u8,
    // Counts how many cycles the instruction has remaining
    pub(crate) cycles: u8,
    // A global accumulation of the number of clocks
    clock_count: u32,
    // Program Counter
    pub pc: u16,
    // Accumulator Register
    pub(crate) a: u8,
    // X Register
    pub(crate) x: u8,
    // Y Register
    pub(crate) y: u8,
    // Stack Pointer (points to location on bus)
    pub(crate) stkp: u8,
    // Status Register
    pub(crate) status: u8,
    // Represents the working input value to the ALU
    fetched: u8,
    lookup: Vec<Instruction>
}

#[allow(dead_code)]
struct Instruction {
    name: String,
    operate: Opcodes,
    addr: AddressModes,
    cycles: usize
}

#[allow(arithmetic_overflow, dead_code)]
impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            state: None,
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

    pub(crate) fn clock(&mut self) -> Result<(), ()> {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.set_flag(U, true);
            self.pc += 1;
            self.cycles = self.lookup[self.opcode as usize].cycles as u8;

            let additional_cycle1 = match self.lookup[self.opcode as usize].addr {
                AddressModes::Imp => self.imp(),
                AddressModes::Imm => self.imm(),
                AddressModes::Rel => self.rel(),
                AddressModes::Abs => self.abs(),
                AddressModes::Izx => self.izx(),
                AddressModes::Izy => self.izy(),
                AddressModes::Zp0 => self.zp0(),
                AddressModes::Zpx => self.zpx(),
                AddressModes::Zpy => self.zpy(),
                AddressModes::Aby => self.aby(),
                AddressModes::Ind => self.ind(),
                AddressModes::Abx => self.abx()
            };

            let additional_cycle2 = match self.lookup[self.opcode as usize].operate {
                Opcodes::Brk => self.brk(),
                Opcodes::Bpl => self.bpl(),
                Opcodes::Jsr => self.jsr(),
                Opcodes::Bmi => self.bmi(),
                Opcodes::Rti => self.rti(),
                Opcodes::Bvc => self.bvc(),
                Opcodes::Rts => self.rts(),
                Opcodes::Bvs => self.bvs(),
                Opcodes::Nop => self.nop(),
                Opcodes::Bcc => self.bcc(),
                Opcodes::Ldy => self.ldy(),
                Opcodes::Bcs => self.bcs(),
                Opcodes::Cpy => self.cpy(),
                Opcodes::Bne => self.bne(),
                Opcodes::Cpx => self.cpx(),
                Opcodes::Beq => self.beq(),
                Opcodes::Ora => self.ora(),
                Opcodes::And => self.and(),
                Opcodes::Eor => self.eor(),
                Opcodes::Adc => self.adc(),
                Opcodes::Sta => self.sta(),
                Opcodes::Lda => self.lda(),
                Opcodes::Cmp => self.cmp(),
                Opcodes::Sbc => self.sbc(),
                Opcodes::Xxx => self.xxx(),
                Opcodes::Ldx => self.ldx(),
                Opcodes::Bit => self.bit(),
                Opcodes::Sty => self.sty(),
                Opcodes::Asl => self.asl(),
                Opcodes::Rol => self.rol(),
                Opcodes::Lsr => self.lsr(),
                Opcodes::Ror => self.ror(),
                Opcodes::Stx => self.stx(),
                Opcodes::Dec => self.dec(),
                Opcodes::Inc => self.inc(),
                Opcodes::Php => self.php(),
                Opcodes::Clc => self.clc(),
                Opcodes::Plp => self.plp(),
                Opcodes::Sec => self.sec(),
                Opcodes::Pha => self.pha(),
                Opcodes::Cli => self.cli(),
                Opcodes::Pla => self.pla(),
                Opcodes::Sei => self.sei(),
                Opcodes::Dey => self.dey(),
                Opcodes::Tya => self.tya(),
                Opcodes::Tay => self.tay(),
                Opcodes::Clv => self.clv(),
                Opcodes::Iny => self.iny(),
                Opcodes::Cld => self.cld(),
                Opcodes::Inx => self.inx(),
                Opcodes::Sed => self.sed(),
                Opcodes::Txa => self.txa(),
                Opcodes::Txs => self.txs(),
                Opcodes::Tax => self.tax(),
                Opcodes::Tsx => self.tsx(),
                Opcodes::Dex => self.dex(),
                Opcodes::Jmp => self.jmp()
            };

            self.cycles += additional_cycle1;
            if additional_cycle2 {
                self.cycles += 1;
            }

            self.set_flag(U, true);
        }
        self.clock_count += 1;
        self.cycles -= 1;

        let mut state = self.state.as_ref().expect("Missing state").as_ref().borrow_mut();
        if self.pc as usize == state.code_end && self.cycles == 0 {
            return Err(());
        }
        return Ok(());
    }

    pub(crate) fn get_state_mut(&mut self) -> RefMut<'_, State> {
        self.state.as_ref().expect("Missing Stete").as_ref().borrow_mut()
    }

    pub(crate) fn get_state(&self) -> Ref<'_, State> {
        self.state.as_ref().expect("Missing Stete").as_ref().borrow()
    }

    fn complete(&mut self) -> bool {
        return self.cycles == 0;
    }

    fn get_flag(&mut self, f: Flags) -> bool {
        return if (self.status & f as u8) > 0 {
            true
        } else {
            false
        }
    }

    pub(crate) fn set_flag(&mut self, f: Flags, v: bool) {
        if v {
            self.status |= f as u8;
        } else {
            self.status &= !(f as u8);
        }
    }

    fn read(&mut self, a: u16) -> u8 {
        let mut state = self.state.as_ref().expect("Missing state").as_ref().borrow_mut();
        cpu_read(&mut state, a, false)
    }

    pub fn write(&mut self, a: u16, d: u8) {
        let mut state = self.state.as_ref().expect("Missing state").as_ref().borrow_mut();
        cpu_write(&mut state, a, d)
    }

    pub fn reset(&mut self) {
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
        if self.get_flag(I) == false {
            self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
            self.stkp -= 1;
            self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;
            self.set_flag(B, false);
            self.set_flag(U, true);
            self.set_flag(I, true);
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
        self.set_flag(B, false);
        self.set_flag(U, true);
        self.set_flag(I, true);

        self.write(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.addr_abs = 0xFFFA;
        let lo = self.read(self.addr_abs + 0);
        let hi = self.read(self.addr_abs + 1);

        self.pc = ((hi << 8) | lo) as u16;

        self.cycles = 8;
    }

    fn imp(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }

    fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc;
        self.pc += 1;
        0
    }

    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpx(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc).wrapping_add(self.x)) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpy(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc).wrapping_add(self.y)) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if self.addr_rel & 0x80 > 0 {
            self.addr_rel |= 0xFF00;
        }
        return 0;
    }

    fn abs(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        return 0;
    }

    fn abx(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        } else {
            return 0;
        }
    }

    fn aby(&mut self) -> u8 {
        let lo = self.read(self.pc) as u16;
        self.pc += 1;
        let hi = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            return 1;
        } else {
            return 0;
        }
    }

    #[allow(arithmetic_overflow)]
    fn ind(&mut self) -> u8 {
        let ptr_lo = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF {
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr + 0) as u16;
        } else {
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr + 0) as u16;
        }

        return 0;
    }

    fn izx(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = (self.read(t + self.x as u16) & 0x00FF) as u16;
        let hi = (self.read(t + self.x as u16 + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        return 0;
    }

    fn izy(&mut self) -> u8 {
        let t = self.read(self.pc) as u16;
        self.pc += 1;

        let lo = self.read(t & 0x00FF) as u16;
        let hi = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs = self.addr_abs.wrapping_add(self.y as u16);

        if self.addr_abs & 0xFF00 != (hi << 8) {
            return 1;
        }

        return 0;
    }

    fn adc(&mut self) -> bool {
        self.fetch();

        let temp = self.a as u16 + self.fetched as u16 + self.get_flag(C) as u16;

        self.set_flag(C, temp > 255);

        self.set_flag(Z, (temp & 0x00FF) == 0);

        self.set_flag(V, (!(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ temp)) & 0x0080 > 0);

        self.set_flag(N, temp & 0x80 > 0);

        self.a = (temp & 0x00FF) as u8;

        return true;
    }

    fn sbc(&mut self) -> bool {
        self.fetch();

        let value = self.fetched as u16 ^ 0x00FF;

        let temp = self.a as u16 + value + self.get_flag(C) as u16;

        self.set_flag(C, (temp & 0xFF00) > 0);

        self.set_flag(Z, (temp & 0x00FF) == 0);

        self.set_flag(V, ((temp ^ self.a as u16) & (temp ^ value) & 0x0080) > 0);

        self.set_flag(N, temp & 0x0080 > 0);

        self.a = (temp & 0x00FF) as u8;

        return true;
    }

    fn and(&mut self) -> bool {
        self.fetch();

        self.a = self.a & self.fetched;

        self.set_flag(Z, self.a == 0x00);

        self.set_flag(N, self.a & 0x80 > 0);

        return true;
    }

    fn asl(&mut self) -> bool {
        self.fetch();

        let temp = (self.fetched as u16) << 1;

        self.set_flag(C, (temp & 0xFF00) > 0);

        self.set_flag(Z, (temp & 0x00FF) == 0x00);

        self.set_flag(N, (temp & 0x80) > 0);

        if self.lookup[self.opcode as usize].addr == AddressModes::Imp {
            self.a = (temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return false;
    }

    fn bcc(&mut self) -> bool {
        if self.get_flag(C) == false {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);
            // let rel = self.addr_rel as i16;
            // if rel > 0 {
            //     self.addr_abs = self.pc + rel.abs() as u16;
            // } else {
            //     self.addr_abs = self.pc - rel.abs() as u16;
            // }

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn bcs(&mut self) -> bool {
        if self.get_flag(C) == true {
            self.cycles += 1;

            let rel = self.addr_rel as i16;
            if rel > 0 {
                self.addr_abs = self.pc + rel.abs() as u16;
            } else {
                self.addr_abs = self.pc - rel.abs() as u16;
            }

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn beq(&mut self) -> bool {
        if self.get_flag(Z) == true {
            self.cycles += 1;
            let rel = self.addr_rel as i16;
            if rel > 0 {
                self.addr_abs = self.pc + rel.abs() as u16;
            } else {
                self.addr_abs = self.pc - rel.abs() as u16;
            }

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn bmi(&mut self) -> bool {
        if self.get_flag(N) == true {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn bne(&mut self) -> bool {
        if self.get_flag(Z) == false {
            self.cycles += 1;
            let rel = self.addr_rel as i16;

            if rel > 0 {
                self.addr_abs = self.pc + rel.abs() as u16;
            } else {
                self.addr_abs = self.pc - rel.abs() as u16;
            }

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn bpl(&mut self) -> bool {
        if self.get_flag(N) == false {
            self.cycles += 1;

            let rel = self.addr_rel as i16;
            if rel > 0 {
                self.addr_abs = self.pc + rel.abs() as u16;
            } else {
                self.addr_abs = self.pc - rel.abs() as u16;
            }

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn bit(&mut self) -> bool {
        self.fetch();

        let temp = self.a & self.fetched;
        self.set_flag(Z, (temp & 0x00FF) == 0x00);
        self.set_flag(N, self.fetched & (1 << 7) > 0);
        self.set_flag(V, self.fetched & (1 << 6) > 0);

        return false;
    }

    fn brk(&mut self) -> bool {
        self.pc += 1;

        self.set_flag(I, true);

        self.write((0x0100 + self.stkp as u16) as u16, ((self.pc >> 8) & 0x00FF) as u8);

        if self.stkp > 0 {
            self.stkp -= 1;
        }

        self.write((0x0100 + self.stkp as u16) as u16, (self.pc & 0x00FF) as u8);
        if self.stkp > 0 {
            self.stkp -= 1;
        }

        self.set_flag(B, true);

        self.write((0x0100 + self.stkp as u16) as u16, self.status);

        if self.stkp > 0 {
            self.stkp -= 1;
        }

        self.set_flag(B, false);

        let a = ((self.read(0xFFFF) as u16) << 8) as u8;

        self.pc = self.read(0xFFFE) as u16 | (a) as u16;

        return false;
    }

    fn bvc(&mut self) -> bool {
        if self.get_flag(V) == false {
            self.cycles += 1;
            let rel = self.addr_rel as i16;

            if rel > 0 {
                self.addr_abs = self.pc + rel.abs() as u16;
            } else {
                self.addr_abs = self.pc - rel.abs() as u16;
            }

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    fn bvs(&mut self) -> bool {
        if self.get_flag(V) == true {
            self.cycles += 1;

            let rel = self.addr_rel as i16;

            if rel > 0 {
                self.addr_abs = self.pc + rel.abs() as u16;
            } else {
                self.addr_abs = self.pc - rel.abs() as u16;
            }

            if self.addr_abs & 0xFF00 != self.pc & 0xFF00 {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }

        return false;
    }

    /**
     * Clear cary
     */
    fn clc(&mut self) -> bool {
        self.set_flag(C, false);
        return false;
    }

    fn cld(&mut self) -> bool {
        self.set_flag(D, false);
        return false;
    }

    fn cli(&mut self) -> bool {
        self.set_flag(I, false);
        return false;
    }

    fn clv(&mut self) -> bool {
        self.set_flag(V, false);
        return false;
    }

    fn cmp(&mut self) -> bool {
        self.fetch();


        let temp = self.a.wrapping_sub(self.fetched);


        self.set_flag(C, self.a >= self.fetched);

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, temp & 0x0080 > 0);

        return true;
    }

    fn cpx(&mut self) -> bool {
        self.fetch();

        let temp = self.x.wrapping_sub(self.fetched);

        self.set_flag(C, self.x >= self.fetched);

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, temp & 0x0080 > 0);

        return false;
    }

    fn cpy(&mut self) -> bool {
        self.fetch();

        let temp = self.y.wrapping_sub(self.fetched);

        self.set_flag(C, self.y >= self.fetched);

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, temp & 0x0080 > 0);

        return false;
    }

    fn dec(&mut self) -> bool {
        self.fetch();

        let temp = self.fetched.wrapping_sub(1);

        self.write(self.addr_abs, temp & 0x00FF);

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, temp & 0x0080 > 0);

        return false;
    }

    fn dex(&mut self) -> bool {
        self.x = self.x.wrapping_sub(1);

        self.set_flag(Z, self.x == 0x00);

        self.set_flag(N, self.x & 0x80 > 0);

        return false;
    }

    fn dey(&mut self) -> bool {
        self.y = self.y.wrapping_sub(1);

        self.set_flag(Z, self.y == 0x00);

        self.set_flag(N, self.y & 0x80 > 0);

        return false;
    }

    fn eor(&mut self) -> bool {
        self.fetch();

        self.a = self.a ^ self.fetched;

        self.set_flag(Z, self.a == 0x00);

        self.set_flag(N, self.a & 0x80 > 0);

        return true;
    }

    fn inc(&mut self) -> bool {
        self.fetch();

        let temp = self.fetched.wrapping_add(1);

        self.write(self.addr_abs, temp & 0x00FF);

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, temp & 0x0080 > 0);

        return false;
    }

    fn inx(&mut self) -> bool {
        self.x = self.x.wrapping_add(1);

        self.set_flag(Z, self.x == 0x00);

        self.set_flag(N, self.x & 0x80 > 0);

        return false;
    }

    fn iny(&mut self) -> bool {
        self.y = self.y.wrapping_add(1);

        self.set_flag(Z, self.y == 0x00);

        self.set_flag(N, self.y & 0x80 > 0);

        return false;
    }

    fn jmp(&mut self) -> bool {
        self.pc = self.addr_abs;
        return false;
    }

    fn jsr(&mut self) -> bool {
        self.pc -= 1;
        self.write(0x0100 + self.stkp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.stkp -= 1;
        self.write(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.pc = self.addr_abs;
        return false;
    }

    /**
    * Load accumulator
    */
    fn lda(&mut self) -> bool {
        self.fetch();

        self.a = self.fetched;

        self.set_flag(Z, self.a == 0x00);

        self.set_flag(N, self.a & 0x80 > 0);

        return true;
    }

    fn ldx(&mut self) -> bool {
        self.fetch();

        self.x = self.fetched;

        self.set_flag(Z, self.x == 0x00);

        self.set_flag(N, self.x & 0x80 > 0);

        return true;
    }

    fn ldy(&mut self) -> bool {
        self.fetch();

        self.y = self.fetched;

        self.set_flag(Z, self.y == 0x00);

        self.set_flag(N, self.y & 0x80 > 0);

        return true;
    }

    fn lsr(&mut self) -> bool {
        self.fetch();

        self.set_flag(C, (self.fetched & 0x0001) > 0);

        let temp = (self.fetched >> 1) as u16;

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, (temp & 0x0080) > 0);

        if self.lookup[self.opcode as usize].addr == AddressModes::Imp {
            self.a = (temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return false;
    }

    fn nop(&mut self) -> bool {
        match self.opcode {
            0x1c | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                return true;
            }
            _ => {}
        }
        return false;
    }

    fn ora(&mut self) -> bool {
        self.fetch();

        self.a = self.a | self.fetched;

        self.set_flag(Z, self.a == 0x00);

        self.set_flag(N, self.a & 0x80 > 0);

        return true;
    }

    fn pha(&mut self) -> bool {
        self.write(0x0100 + self.stkp as u16, self.a);
        self.stkp -= 1;

        return false;
    }

    fn php(&mut self) -> bool {
        self.write(0x0100 + self.stkp as u16, self.status | B as u8 | U as u8);
        self.set_flag(B, false);
        self.set_flag(U, false);
        self.stkp -= 1;

        return false;
    }

    fn pla(&mut self) -> bool {
        self.stkp += 1;
        self.a = self.read(0x0100 + self.stkp as u16);
        self.set_flag(Z, self.a == 0x00);
        self.set_flag(N, self.a & 0x80 > 0);

        return false;
    }

    fn plp(&mut self) -> bool {
        self.stkp += 1;
        self.status = self.read(0x0100 + self.stkp as u16);
        self.set_flag(U, true);
        return false;
    }

    fn rol(&mut self) -> bool {
        self.fetch();

        let temp = (self.fetched as u16) << 1 | self.get_flag(C) as u16;

        self.set_flag(C, (temp & 0xFF00) > 0);

        self.set_flag(Z, (temp & 0x00FF) == 0x0000);

        self.set_flag(N, (temp & 0x0080) > 0);

        if self.lookup[self.opcode as usize].addr == AddressModes::Imp {
            self.a = (temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return false;
    }

    fn ror(&mut self) -> bool {
        self.fetch();

        let temp = ((self.get_flag(C) as u16) << 7) | (self.fetched as u16 >> 1);

        self.set_flag(C, (self.fetched & 0x01) > 0);

        self.set_flag(Z, (temp & 0x00FF) == 0x00);

        self.set_flag(N, (temp & 0x0080) > 0);

        if self.lookup[self.opcode as usize].addr == AddressModes::Imp {
            self.a = (temp & 0x00FF) as u8;
        } else {
            self.write(self.addr_abs, (temp & 0x00FF) as u8);
        }

        return false;
    }

    fn rti(&mut self) -> bool {
        self.stkp += 1;
        self.status = self.read(0x0100 + self.stkp as u16);
        self.status &= !(B as u8);
        self.status &= !(U as u8);
        self.stkp += 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;
        self.pc |= (self.read(0x0100 + self.stkp as u16) as u16) << 8;

        return false;
    }

    /**
    * Return to subroutine
    * https://sites.google.com/site/6502asembly/6502-instruction-set/rts
    uint8_t olc6502::RTS()
    {
    	stkp++;
    	pc = (uint16_t)read(0x0100 + stkp);
    	stkp++;
    	pc |= (uint16_t)read(0x0100 + stkp) << 8;

    	pc++;
    	return 0;
    }
    */
    fn rts(&mut self) -> bool {
        self.stkp += 1;
        self.pc = self.read(0x0100 + self.stkp as u16) as u16;
        self.stkp += 1;

        let tmp = (self.read(0x0100 + self.stkp as u16) as u16) << 8;
        self.pc |= tmp;

        self.pc += 1;

        return false;
    }

    fn sec(&mut self) -> bool {
        self.set_flag(C, true);
        return false;
    }

    fn sed(&mut self) -> bool {
        self.set_flag(D, true);
        return false;
    }

    fn sei(&mut self) -> bool {
        self.set_flag(I, true);
        return false;
    }

    fn sta(&mut self) -> bool {
        self.write(self.addr_abs, self.a);
        return false;
    }

    fn stx(&mut self) -> bool {
        self.write(self.addr_abs, self.x);
        return false;
    }

    fn sty(&mut self) -> bool {
        self.write(self.addr_abs, self.y);
        return false;
    }

    fn tax(&mut self) -> bool {
        self.x = self.a;
        self.set_flag(Z, self.x == 0x00);
        self.set_flag(N, (self.x & 0x80) > 0);
        return false;
    }

    fn tay(&mut self) -> bool {
        self.y = self.a;
        self.set_flag(Z, self.y == 0x00);
        self.set_flag(N, (self.y & 0x80) > 0);
        return false;
    }

    fn tsx(&mut self) -> bool {
        self.x = self.stkp;
        self.set_flag(Z, self.x == 0x00);
        self.set_flag(N, (self.x & 0x80) > 0);
        return false;
    }

    fn txa(&mut self) -> bool {
        self.a = self.x;
        self.set_flag(Z, self.a == 0x00);
        self.set_flag(N, (self.a & 0x80) > 0);
        return false;
    }

    fn txs(&mut self) -> bool {
        self.stkp = self.x;
        return false;
    }

    fn tya(&mut self) -> bool {
        self.a = self.y;
        self.set_flag(Z, self.a == 0x00);
        self.set_flag(N, (self.a & 0x80) > 0);
        return false;
    }

    fn xxx(&mut self) -> bool {
        return false;
    }

    pub(crate) fn disassemble(&mut self) -> HashMap<u16, String> {
        let mut addr: u32 = 0;
        let stop_addr = 0xFFFF;

        let mut map = HashMap::new();

        while addr <= stop_addr {
            let line_addr = addr;

            let hex_str = hex::encode(&addr.to_be_bytes());
            let mut dis_string = format!("${}: ", &hex_str[hex_str.len()-4..]);

            let opcode = self.read(addr as u16);
            addr += 1;

            dis_string = format!("{}{} ", dis_string, self.lookup[opcode as usize].name);

            if self.lookup[opcode as usize].addr == AddressModes::Imp {
                dis_string = format!("{}{}", dis_string, "{IMP}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Imm {
                let value = self.read(addr as u16);
                addr += 1;
                dis_string = format!("{}#${} {}", dis_string, hex::encode(&value.to_be_bytes()), "{IMM}")
            } else if self.lookup[opcode as usize].addr == AddressModes::Zp0 {
                let lo = self.read(addr as u16);
                addr += 1;
                dis_string = format!("{}${} {}", dis_string, hex::encode(&lo.to_be_bytes()), "{ZP0}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Zpx {
                let lo = self.read(addr as u16);
                addr += 1;
                dis_string = format!("{}${}, X {}", dis_string, hex::encode(&lo.to_be_bytes()), "{ZPX}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Zpy {
                let lo = self.read(addr as u16);
                addr += 1;
                dis_string = format!("{}${}, Y {}", dis_string, hex::encode(&lo.to_be_bytes()), "{ZPY}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Izx {
                let lo = self.read(addr as u16);
                addr += 1;
                dis_string = format!("{}(${}, X) {}", dis_string, hex::encode(&lo.to_be_bytes()), "{IZX}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Izy {
                let lo = self.read(addr as u16);
                addr += 1;
                dis_string = format!("{}(${}, Y) {}", dis_string, hex::encode(&lo.to_be_bytes()), "{IZY}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Abs {
                let lo = self.read(addr as u16) as u16;
                addr += 1;
                let hi = self.read(addr as u16) as u16;
                addr += 1;
                let value = (hi << 8 | lo) as u8;
                dis_string = format!("{}${} {}", dis_string, hex::encode(&value.to_be_bytes()), "{ABS}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Abx {
                let lo = self.read(addr as u16);
                addr += 1;
                let hi = self.read(addr as u16);
                addr += 1;
                let value = (((hi as u16) << 8) | lo as u16) as u8;
                dis_string = format!("{}${}, X {}", dis_string, hex::encode(&value.to_be_bytes()), "{ABX}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Aby {
                let lo = self.read(addr as u16);
                addr += 1;
                let hi = self.read(addr as u16);
                addr += 1;
                let value = (((hi as u16) << 8) | lo as u16) as u8;
                dis_string = format!("{}${}, Y {}", dis_string, hex::encode(&value.to_be_bytes()), "{ABY}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Ind {
                let lo = self.read(addr as u16);
                addr += 1;
                let hi = self.read(addr as u16);
                addr += 1;
                let value = (((hi as u16) << 8) | lo as u16) as u8;
                dis_string = format!("{}(${}) {}", dis_string, hex::encode(&value.to_be_bytes()), "{IND}");
            } else if self.lookup[opcode as usize].addr == AddressModes::Rel {
                let value = self.read(addr as u16);
                addr += 1;
                let addr_value = addr as u16 + value as u16;
                dis_string = format!("{}${} [${}] {}",
                                     dis_string,
                                     hex::encode(&value.to_be_bytes()),
                                     hex::encode(&addr_value.to_be_bytes()),
                                     "{REL}");
            }
            map.insert(line_addr as u16, dis_string);
        }
        map
    }
}