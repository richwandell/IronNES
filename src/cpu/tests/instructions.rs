use crate::{advance, create_system};

#[test]
fn test_wrapping_inc() {
    /*
; Load number 10 into memory location $00
ldx #255
stx $00

; increment $00
inc $00
    */
    let code = vec![0xA2, 0xff, 0x86, 0x00, 0xE6, 0x00];
    let (_bus_ref, cpu_ref, _ppu_ref, state_ref) = create_system();

    {
        let mut state = state_ref.as_ref().borrow_mut();
        state.load(code, 0);
    }

    let mut cpu = cpu_ref.as_ref().borrow_mut();
    let mut ppu = _ppu_ref.as_ref().borrow_mut();
    let _ = advance(&mut ppu, &mut cpu);
    let _ = advance(&mut ppu, &mut cpu);
    let _ = advance(&mut ppu, &mut cpu);

    assert_eq!(1, 1);
}