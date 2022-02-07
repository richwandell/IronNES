use crate::cpu::{AddressModes, Opcodes};
use crate::cpu::cpu_6502::Instruction;

pub(crate) fn make_instructions() -> Vec<Instruction> {
    vec![
        Instruction { name: "Brk".to_string(), operate: Opcodes::Brk, addr: AddressModes::Imm, cycles: 7 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imm, cycles: 3 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Zp0, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "Php".to_string(), operate: Opcodes::Php, addr: AddressModes::Imp, cycles: 3 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Bpl".to_string(), operate: Opcodes::Bpl, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Zpx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Clc".to_string(), operate: Opcodes::Clc, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Ora".to_string(), operate: Opcodes::Ora, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Asl".to_string(), operate: Opcodes::Asl, addr: AddressModes::Abx, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "Jsr".to_string(), operate: Opcodes::Jsr, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "Bit".to_string(), operate: Opcodes::Bit, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Zp0, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "Plp".to_string(), operate: Opcodes::Plp, addr: AddressModes::Imp, cycles: 4 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Bit".to_string(), operate: Opcodes::Bit, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Bmi".to_string(), operate: Opcodes::Bmi, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Zpx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Sec".to_string(), operate: Opcodes::Sec, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "And".to_string(), operate: Opcodes::And, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Rol".to_string(), operate: Opcodes::Rol, addr: AddressModes::Abx, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "Rti".to_string(), operate: Opcodes::Rti, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imm, cycles: 3 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Zp0, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "Pha".to_string(), operate: Opcodes::Pha, addr: AddressModes::Imp, cycles: 3 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Jmp".to_string(), operate: Opcodes::Jmp, addr: AddressModes::Abs, cycles: 3 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Bvc".to_string(), operate: Opcodes::Bvc, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Zpx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Cli".to_string(), operate: Opcodes::Cli, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Eor".to_string(), operate: Opcodes::Eor, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Lsr".to_string(), operate: Opcodes::Lsr, addr: AddressModes::Abx, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "Rts".to_string(), operate: Opcodes::Rts, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imm, cycles: 3 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Zp0, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "Pla".to_string(), operate: Opcodes::Pla, addr: AddressModes::Imp, cycles: 4 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Jmp".to_string(), operate: Opcodes::Jmp, addr: AddressModes::Ind, cycles: 5 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Bvs".to_string(), operate: Opcodes::Bvs, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 8 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Zpx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Sei".to_string(), operate: Opcodes::Sei, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Adc".to_string(), operate: Opcodes::Adc, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Ror".to_string(), operate: Opcodes::Ror, addr: AddressModes::Abx, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 7 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Sax".to_string(), operate: Opcodes::Sax, addr: AddressModes::Izx, cycles: 6 }, //OpCode::new(0x83, "*SAX", 2, 6, AddressingMode::Indirect_X),
        Instruction { name: "Sty".to_string(), operate: Opcodes::Sty, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Stx".to_string(), operate: Opcodes::Stx, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "*Sax".to_string(), operate: Opcodes::Sax, addr: AddressModes::Zp0, cycles: 3 }, //OpCode::new(0x87, "*SAX", 2, 3, AddressingMode::ZeroPage),
        Instruction { name: "Dey".to_string(), operate: Opcodes::Dey, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Txa".to_string(), operate: Opcodes::Txa, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Sty".to_string(), operate: Opcodes::Sty, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Stx".to_string(), operate: Opcodes::Stx, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "*Sax".to_string(), operate: Opcodes::Sax, addr: AddressModes::Abs, cycles: 4 }, //OpCode::new(0x8f, "*SAX", 3, 4, AddressingMode::Absolute),
        Instruction { name: "Bcc".to_string(), operate: Opcodes::Bcc, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Izy, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 6 },
        Instruction { name: "Sty".to_string(), operate: Opcodes::Sty, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Stx".to_string(), operate: Opcodes::Stx, addr: AddressModes::Zpy, cycles: 4 },
        Instruction { name: "*Sax".to_string(), operate: Opcodes::Sax, addr: AddressModes::Zpy, cycles: 4 }, //OpCode::new(0x97, "*SAX", 2, 4, AddressingMode::ZeroPage_Y),
        Instruction { name: "Tya".to_string(), operate: Opcodes::Tya, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Aby, cycles: 5 },
        Instruction { name: "Txs".to_string(), operate: Opcodes::Txs, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "Sta".to_string(), operate: Opcodes::Sta, addr: AddressModes::Abx, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 5 },
        Instruction { name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "*Lax".to_string(), operate: Opcodes::Lax, addr: AddressModes::Izx, cycles: 6 }, //OpCode::new(0xa3, "*LAX", 2, 6, AddressingMode::Indirect_X),
        Instruction { name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "*Lax".to_string(), operate: Opcodes::Lax, addr: AddressModes::Zp0, cycles: 3 }, //OpCode::new(0xa7, "*LAX", 2, 3, AddressingMode::ZeroPage),
        Instruction { name: "Tay".to_string(), operate: Opcodes::Tay, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Tax".to_string(), operate: Opcodes::Tax, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "*Lax".to_string(), operate: Opcodes::Lax, addr: AddressModes::Abs, cycles: 4 }, //OpCode::new(0xaf, "*LAX", 3, 4, AddressingMode::Absolute),
        Instruction { name: "Bcs".to_string(), operate: Opcodes::Bcs, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Lax".to_string(), operate: Opcodes::Lax, addr: AddressModes::Izy, cycles: 5 }, //OpCode::new(0xb3, "*LAX", 2, 5, AddressingMode::Indirect_Y),
        Instruction { name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Zpy, cycles: 4 },
        Instruction { name: "*Lax".to_string(), operate: Opcodes::Lax, addr: AddressModes::Zpy, cycles: 4 }, //OpCode::new(0xb7, "*LAX", 2, 4, AddressingMode::ZeroPage_Y),
        Instruction { name: "Clv".to_string(), operate: Opcodes::Clv, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "Tsx".to_string(), operate: Opcodes::Tsx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 4 },
        Instruction { name: "Ldy".to_string(), operate: Opcodes::Ldy, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Lda".to_string(), operate: Opcodes::Lda, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Ldx".to_string(), operate: Opcodes::Ldx, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "*Lax".to_string(), operate: Opcodes::Lax, addr: AddressModes::Aby, cycles: 4 }, //OpCode::new(0xbf, "*LAX", 3, 4, AddressingMode::Absolute_Y),
        Instruction { name: "Cpy".to_string(), operate: Opcodes::Cpy, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Izx, cycles: 8 }, //OpCode::new(0xc3, "*DCP", 2, 8, AddressingMode::Indirect_X),
        Instruction { name: "Cpy".to_string(), operate: Opcodes::Cpy, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Zp0, cycles: 5 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Zp0, cycles: 5 }, //OpCode::new(0xc7, "*DCP", 2, 5, AddressingMode::ZeroPage),
        Instruction { name: "Iny".to_string(), operate: Opcodes::Iny, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Dex".to_string(), operate: Opcodes::Dex, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Cpy".to_string(), operate: Opcodes::Cpy, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Abs, cycles: 6 }, //OpCode::new(0xCF, "*DCP", 3, 6, AddressingMode::Absolute),
        Instruction { name: "Bne".to_string(), operate: Opcodes::Bne, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Izy, cycles: 8 }, //OpCode::new(0xd3, "*DCP", 2, 8, AddressingMode::Indirect_Y),
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Zpx, cycles: 6 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Zpx, cycles: 6 }, //OpCode::new(0xd7, "*DCP", 2, 6, AddressingMode::ZeroPage_X)
        Instruction { name: "Cld".to_string(), operate: Opcodes::Cld, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "Nop".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Aby, cycles: 7 }, //OpCode::new(0xdb, "*DCP", 3, 7, AddressingMode::Absolute_Y),
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Cmp".to_string(), operate: Opcodes::Cmp, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Dec".to_string(), operate: Opcodes::Dec, addr: AddressModes::Abx, cycles: 7 },
        Instruction { name: "*Dcp".to_string(), operate: Opcodes::Dcp, addr: AddressModes::Abx, cycles: 7 }, // OpCode::new(0xdF, "*DCP", 3, 7, AddressingMode::Absolute_X),
        Instruction { name: "Cpx".to_string(), operate: Opcodes::Cpx, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Izx, cycles: 6 },
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Izx, cycles: 8 }, //OpCode::new(0xe3, "*ISB", 2,8, AddressingMode::Indirect_X),
        Instruction { name: "Cpx".to_string(), operate: Opcodes::Cpx, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Zp0, cycles: 3 },
        Instruction { name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Zp0, cycles: 5 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Zp0, cycles: 5 }, //OpCode::new(0xe7, "*ISB", 2,5, AddressingMode::ZeroPage),
        Instruction { name: "Inx".to_string(), operate: Opcodes::Inx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Imm, cycles: 2 },
        Instruction { name: "Nop".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Imm, cycles: 2 }, //OpCode::new(0xeb, "*SBC", 2,2, AddressingMode::Immediate),
        Instruction { name: "Cpx".to_string(), operate: Opcodes::Cpx, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Abs, cycles: 4 },
        Instruction { name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Abs, cycles: 6 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Abs, cycles: 6 }, //OpCode::new(0xef, "*ISB", 3,6, AddressingMode::Absolute),
        Instruction { name: "Beq".to_string(), operate: Opcodes::Beq, addr: AddressModes::Rel, cycles: 2 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Izy, cycles: 5 },
        Instruction { name: "???".to_string(), operate: Opcodes::Xxx, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Izy, cycles: 8 }, //OpCode::new(0xf3, "*ISB", 2,8, AddressingMode::Indirect_Y),
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Zpx, cycles: 4 },
        Instruction { name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Zpx, cycles: 6 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Zpx, cycles: 6 }, //OpCode::new(0xf7, "*ISB", 2,6, AddressingMode::ZeroPage_X),
        Instruction { name: "Sed".to_string(), operate: Opcodes::Sed, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Aby, cycles: 4 },
        Instruction { name: "Nop".to_string(), operate: Opcodes::Nop, addr: AddressModes::Imp, cycles: 2 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Aby, cycles: 7 }, //OpCode::new(0xfb, "*ISB", 3,7, AddressingMode::Absolute_Y),
        Instruction { name: "???".to_string(), operate: Opcodes::Nop, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Sbc".to_string(), operate: Opcodes::Sbc, addr: AddressModes::Abx, cycles: 4 },
        Instruction { name: "Inc".to_string(), operate: Opcodes::Inc, addr: AddressModes::Abx, cycles: 7 },
        Instruction { name: "*Isb".to_string(), operate: Opcodes::Isb, addr: AddressModes::Abx, cycles: 7 }, //OpCode::new(0xff, "*ISB", 3,7, AddressingMode::Absolute_X),
    ]
}