

use std::fs::File;
use std::io::Read;
use piston::EventSettings;
use nes_emulator::{create_system};
use nes_emulator::display::display::Game;
use nes_emulator::display::display_debug::NesDebug;


fn main() {
    let mut file = File::open("assets/nestest.nes").expect("File not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let file_slice = &buffer[0x0010..0x4000];
    let (bus_ref, cpu_ref, ppu_ref, state_ref) = create_system();

    {
        let mut state = state_ref.as_ref().borrow_mut();
        state.load(file_slice.to_vec(), 0xC000);
        state.load(file_slice.to_vec(), 0x8000);
    }
    {
        let mut cpu = cpu_ref.as_ref().borrow_mut();
        cpu.reset();
        cpu.pc = 0x0c000;
    }

    let mut game = NesDebug::new(
        state_ref.clone(),
        cpu_ref.clone(),
        ppu_ref.clone(),
        vec![0x0000, 0x0600, 0xC600],
        EventSettings {
            max_fps: 60,
            ups: 100,
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
