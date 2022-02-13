pub(crate) mod cpu_6502;
mod tests;
mod instructions;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Flags
{
    // Carry Bit
    C = (1 << 0),
    // Zero
    Z = (1 << 1),
    // Disable Interrupts
    I = (1 << 2),
    // Decimal Mode (unused in this implementation)
    D = (1 << 3),
    // Break
    B = (1 << 4),
    // Unused
    U = (1 << 5),
    // Overflow
    V = (1 << 6),
    // Negative
    N = (1 << 7),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Opcodes {
    Brk,
    Bpl,
    Jsr,
    Bmi,
    Rti,
    Bvc,
    Rts,
    Bvs,
    Nop,
    Bcc,
    Ldy,
    Bcs,
    Cpy,
    Bne,
    Cpx,
    Beq,
    Ora,
    And,
    Eor,
    Adc,
    Sta,
    Lda,
    Lax,
    Cmp,
    Sbc,
    Xxx,
    Ldx,
    Bit,
    Sty,
    Asl,
    Slo,
    Rol,
    Lsr,
    Ror,
    Stx,
    Sax,
    Dec,
    Dcp,
    Inc,
    Isb,
    Php,
    Clc,
    Plp,
    Sec,
    Pha,
    Cli,
    Pla,
    Sei,
    Dey,
    Tya,
    Tay,
    Clv,
    Iny,
    Cld,
    Inx,
    Sed,
    Txa,
    Txs,
    Tax,
    Tsx,
    Dex,
    Jmp
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum AddressModes {
    Imp,
    // Immediate
    Imm,
    Rel,
    Abs,
    // Indirect X
    Izx,
    // Indirect Y
    Izy,
    // Zero Page
    Zp0,
    Zpx,
    Zpy,
    Aby,
    Ind,
    Abx
}


use crate::bus::clock;
use crate::cpu::cpu_6502::Cpu;
use crate::ppu::Ppu;
use crate::state::State;


pub(crate) fn cpu_loop(cpu: &mut Cpu) {
    loop {
        if let Err(_) = cpu.clock() {
            break
        }
    }
}

pub(crate) fn real_loop(ppu: &mut Ppu, cpu: &mut Cpu) {
    loop {
        if let Err(_) = clock(ppu, cpu) {
            break
        }
    }
}