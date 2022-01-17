use crate::bus::cpu_write;

pub struct State {
    pub(crate) cpu_ram: Vec<u8>,
    pub(crate) ppu_ram: Vec<u8>,
    pub(crate) code_end: usize,
    pub(crate) name_tables: Vec<Vec<u8>>,
    pub(crate) palette_table: Vec<u8>,
    pub(crate) n_system_clock_counter: usize
}

impl State {
    pub fn new() -> State {
        State {
            cpu_ram: vec![0; 2048],
            ppu_ram: vec![0; 2048],
            code_end: 0,
            name_tables: vec![vec![0; 1024], vec![0; 1024]],
            palette_table: vec![0; 32],
            n_system_clock_counter: 0
        }
    }

    pub(crate) fn load(&mut self, code: Vec<u8>) {
        let end = code.len().clone();
        self.code_end = end;

        let mut i = 0;
        for item in code {
            cpu_write(self, i, item);
            i += 1;
        }
    }
}