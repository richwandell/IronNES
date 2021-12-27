pub(crate) mod cpu_6502;
mod tests;

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
enum Opcodes {
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
    Cmp,
    Sbc,
    Xxx,
    Ldx,
    Bit,
    Sty,
    Asl,
    Rol,
    Lsr,
    Ror,
    Stx,
    Dec,
    Inc,
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
enum AddressModes {
    Imp,
    Imm,
    Rel,
    Abs,
    Izx,
    Izy,
    Zp0,
    Zpx,
    Zpy,
    Aby,
    Ind,
    Abx
}

macro_rules! create_cpu {
    ($var1:ident) => {
        let mut ppu = crate::ppu::Ppu::new();
        let mut bus = crate::bus::Bus::new(&mut ppu);
        let mut $var1 = crate::cpu::cpu_6502::Cpu::new(&mut bus);
    };
}

pub(crate) use create_cpu;