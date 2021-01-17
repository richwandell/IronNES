mod Cpu6502;
mod fn_ptr;

#[derive(Clone, Debug, PartialEq)]
enum Flags
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
    brk,
    bpl,
    jsr,
    bmi,
    rti,
    bvc,
    rts,
    bvs,
    nop,
    bcc,
    ldy,
    bcs,
    cpy,
    bne,
    cpx,
    beq,
    ora,
    and,
    eor,
    adc,
    sta,
    lda,
    cmp,
    sbc,
    xxx,
    ldx,
    bit,
    sty,
    asl,
    rol,
    lsr,
    ror,
    stx,
    dec,
    inc,
    php,
    clc,
    plp,
    sec,
    pha,
    cli,
    pla,
    sei,
    dey,
    tya,
    tay,
    clv,
    iny,
    cld,
    inx,
    sed,
    txa,
    txs,
    tax,
    tsx,
    dex,
    jmp
}

#[derive(Clone, Debug, PartialEq)]
enum AddressModes {
    imp,
    imm,
    rel,
    abs,
    izx,
    izy,
    zp0,
    zpx,
    zpy,
    aby,
    ind,
    abx
}