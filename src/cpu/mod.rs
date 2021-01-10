mod Cpu6502;
mod fn_ptr;

enum FLags
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