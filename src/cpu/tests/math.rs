use crate::bus::Bus;
use crate::cpu::cpu_6502::Cpu;
use crate::cpu::tests::create_cpu;

#[test]
fn test_increment_and_decrement_numbers() {
    /*
; Assembly
; Load number 10 into memory location $00 and $01
ldx #10
stx $00
stx $01

; increment $00 and decrement $01
inc $00
dec $01

; move into $0300 and $0301
ldx $00
stx $0300
ldx $01
stx $0301

     */

    let code: Vec<u8> = vec![
        0xA2, 0x0A, 0x86, 0x00, 0x86, 0x01, 0xE6, 0x00,
        0xC6, 0x01, 0xA6, 0x00, 0x8E, 0x00, 0x03, 0xA6,
        0x01, 0x8E, 0x01, 0x03
    ];
    let end = code.len().clone();
    let mut cpu = create_cpu();
    cpu.bus.load(code);

    loop {
        cpu.clock();
        if cpu.pc as usize == end && cpu.cycles == 0 {
            break;
        }
    }

    assert_eq!(cpu.bus.cpu_ram[0], 11);
    assert_eq!(cpu.bus.cpu_ram[1], 9);
    assert_eq!(cpu.bus.cpu_ram[768], 11);
    assert_eq!(cpu.bus.cpu_ram[769], 9);
}

#[test]
fn test_add_10_and_20() {
    /*
; load 10 into $00 and 20 into $01
ldx #10
stx $00
ldx #20
stx $01

; add $00 and $01
lda $00
clc
adc $01

; store the first byte of the result in $02
sta $02

; add the cary bit to zero and store it in $03
lda #0
adc #0
sta $03
     */
    let code: Vec<u8> = vec![
        0xA2, 0x0A, 0x86, 0x00, 0xA2, 0x14, 0x86, 0x01,
        0xA5, 0x00, 0x18, 0x65, 0x01, 0x85, 0x02, 0xA9,
        0x00, 0x69, 0x00, 0x85, 0x03
    ];
    let end = code.len().clone();
    let mut cpu = create_cpu();
    cpu.bus.load(code);

    loop {
        cpu.clock();
        if cpu.pc as usize == end && cpu.cycles == 0 {
            break;
        }
    }

    assert_eq!(cpu.bus.cpu_ram[2], 30);
    assert_eq!(cpu.bus.cpu_ram[3], 0);
}
