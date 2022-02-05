use nes_emulator::{advance, create_system, Display};

fn main() {

    let (_bus_ref, cpu_ref, ppu_ref, state_ref) = create_system();

    // let code: Vec<u8> = vec![
    //     0xA2, 0x0A, 0x86, 0x00, 0x86, 0x01, 0xE6, 0x00,
    //     0xC6, 0x01, 0xA6, 0x00, 0x8E, 0x00, 0x03, 0xA6,
    //     0x01, 0x8E, 0x01, 0x03
    // ];

    let code : Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC,
                              0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];

    state_ref.as_ref().borrow_mut().load(code, 0x8000);
    cpu_ref.as_ref().borrow_mut().reset();

    let mut display = Display::debug(state_ref.clone(), cpu_ref.clone());

    display.start(|event| {
        let mut ppu= ppu_ref.as_ref().borrow_mut();
        let mut cpu = cpu_ref.as_ref().borrow_mut();

        if let Some(_args) = event {
            if let Ok(_) = advance(&mut ppu, &mut cpu) {
                println!("{}", "clock ok");
            } else {
                println!("{}", "clock not ok")
            }
        }
    });
}