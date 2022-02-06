use std::cell::RefCell;
use std::rc::Rc;
use piston::{EventSettings};
use nes_emulator::{advance, create_system};
use rand::{Rng, thread_rng};
use nes_emulator::display::display::Game;
use nes_emulator::display::display_debug::NesDebug;


fn main() {
    let game_code : Vec<u8> = vec![0xA2, 0x0A, 0x8E, 0x00, 0x00, 0xA2, 0x03, 0x8E, 0x01, 0x00, 0xAC,
                                              0x00, 0x00, 0xA9, 0x00, 0x18, 0x6D, 0x01, 0x00, 0x88, 0xD0, 0xFA, 0x8D, 0x02, 0x00, 0xEA, 0xEA, 0xEA];

    let (_bus_ref, cpu_ref, ppu_ref, state_ref) = create_system();

    state_ref.as_ref().borrow_mut().load(game_code, 0);
    cpu_ref.as_ref().borrow_mut().reset();

    let mut display= NesDebug::new(
        state_ref.clone(),
        cpu_ref.clone(),
        ppu_ref.clone(),
        vec![0x0000, 0x8000]
    );
    display.start();
}

