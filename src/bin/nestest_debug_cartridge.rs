use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use piston::EventSettings;
use nes_emulator::{create_system};
use nes_emulator::display::display::Game;
use nes_emulator::display::display_debug::NesDebug;
use nes_emulator::cartridge::Cartridge;

fn main() {
    let cart = Cartridge::new("assets/nestest.nes");
    let (_bus_ref, cpu_ref, ppu_ref, state_ref) = create_system();

    {
        let mut state = state_ref.as_ref().borrow_mut();
        state.connect_cartridge(Some(Rc::new(RefCell::new(cart))));
    }
    {
        let mut cpu = cpu_ref.as_ref().borrow_mut();
        cpu.reset();
    }

    let mut game = NesDebug::new(
        state_ref.clone(),
        cpu_ref.clone(),
        ppu_ref.clone(),
        vec![0x0000, 0x8000],
        EventSettings {
            max_fps: 60,
            ups: 1000,
            swap_buffers: true,
            bench_mode: false,
            lazy: false,
            ups_reset: 0,
        },
        0x0000,
        0xC66E
    );
    game.start();
}
