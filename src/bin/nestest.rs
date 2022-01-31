

use std::fs::File;
use std::io::Read;
use nes_emulator::{advance, create_system, Display};


fn main() {
    let mut file = File::open("assets/nestest.nes").expect("File not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let file_slice = &buffer[0x0010..0x4000];
    let (bus_ref, cpu_ref, ppu_ref, state_ref) = create_system();

    state_ref.as_ref().borrow_mut().load(file_slice.to_vec(), 0xC000);
    state_ref.as_ref().borrow_mut().load(file_slice.to_vec(), 0x8000);
    cpu_ref.as_ref().borrow_mut().reset();

    let mut display = Display::new(state_ref.clone(), cpu_ref.clone());

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
