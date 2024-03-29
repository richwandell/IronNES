use std::borrow::BorrowMut;



use crate::cpu::{real_loop};
use crate::cpu::tests::create_devices;



fn run_test_code(code: Vec<u8>, offset: u16) -> Vec<u8> {
    create_devices!(ppu, cpu);
    cpu.get_state_mut().load(code, 0);
    real_loop(&mut ppu, &mut cpu);
    let state = cpu.get_state();
    state.cpu_ram.clone()
}

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
    let cpu_ram = run_test_code(code, 0);
    assert_eq!(cpu_ram[0], 11);
    assert_eq!(cpu_ram[1], 9);
    assert_eq!(cpu_ram[768], 11);
    assert_eq!(cpu_ram[769], 9);
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
    let cpu_ram = run_test_code(code, 0);
    assert_eq!(cpu_ram[2], 30);
    assert_eq!(cpu_ram[3], 0);
}

#[test]
fn test_add_8_bit_number_with_carry() {
    /*
; (240 + 20 = 260)
clc

; low
lda #20
adc #240
sta $00

; high
lda #00
adc #00

sta $01
     */
    let code: Vec<u8> = vec![
        0x18, 0xA9, 0x14, 0x69, 0xF0, 0x85, 0x00, 0xA9,
        0x00, 0x69, 0x00, 0x85, 0x01,
    ];
    let cpu_ram = run_test_code(code, 0);
    assert_eq!(cpu_ram[0], 4);
    assert_eq!(cpu_ram[1], 1);
    let final_num = 256 * cpu_ram[1] as u16 + (cpu_ram[0] as u16);
    assert_eq!(final_num, 260);
}

#[test]
fn test_add_16_bit_number() {
    /*
clc

; num1 = 500
; num2 = 700

; low
lda #244
adc #188
sta $00

; high
lda #01
adc #02

sta $01
     */
    let code: Vec<u8> = vec![
        0x18, 0xA9, 0xF4, 0x69, 0xBC, 0x85, 0x00, 0xA9,
        0x01, 0x69, 0x02, 0x85, 0x01,
    ];
    let cpu_ram = run_test_code(code, 0);
    let final_num = 256 * cpu_ram[1] as u16 + (cpu_ram[0] as u16);
    assert_eq!(final_num, 1200);
}

#[test]
fn test_subtract_16_bit_number() {
    /*
; num1 = 700
; num2 = 500

sec
; low
lda #188
sbc #244
sta $00

; high
lda #02
sbc #01
sta $01
     */
    let code: Vec<u8> = vec![
        0x38, 0xA9, 0xBC, 0xE9, 0xF4, 0x85, 0x00, 0xA9,
        0x02, 0xE9, 0x01, 0x85, 0x01,
    ];
    let cpu_ram = run_test_code(code, 0);
    let final_num = 256 * cpu_ram[1] as u16 + (cpu_ram[0] as u16);
    assert_eq!(final_num, 200);
}

#[test]
fn test_multiply_10_and_3() {
    /*
        *=$8000
        LDX #10
        STX $0000
        LDX #3
        STX $0001
        LDY $0000
        LDA #0
        CLC
        loop
        ADC $0001
        DEY
        BNE loop
        STA $0002
        NOP
        NOP
        NOP
    */
    let code : Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC,
                              0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];
    let cpu_ram = run_test_code(code, 0x8000);
    assert_eq!(cpu_ram[0], 10);
    assert_eq!(cpu_ram[1], 3);
    assert_eq!(cpu_ram[2], 30);
}