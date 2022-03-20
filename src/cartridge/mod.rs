mod tests;

use std::fs::File;
use std::io::Read;

struct Header {
    name: Vec<char>,
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: Vec<u8>
}

pub struct Cartridge {
    header: Header,
    pub(crate) n_mapper_id: u8,
    pub(crate) n_prgbanks: u8,
    n_chrbanks: u8,
    pub(crate) v_prg_memory: Vec<u8>,
    pub(crate) v_chr_memory: Vec<u8>
}


impl Cartridge {

    pub fn new(file_path: &str) -> Cartridge {
        let mut file = File::open(file_path).expect("File not found");
        let mut buffer: [u8; 16] = [0; 16];
        file.read(&mut buffer).expect("Cannot read from cartridge file");

        let header = Header {
            name: vec![buffer[0] as char, buffer[1] as char, buffer[2] as char, buffer[3] as char],
            prg_rom_chunks: buffer[4],
            chr_rom_chunks: buffer[5],
            mapper1: buffer[6],
            mapper2: buffer[7],
            prg_ram_size: buffer[8],
            tv_system1: buffer[9],
            tv_system2: buffer[10],
            unused: vec![buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]]
        };


        if (header.mapper1 & 0x04) > 0x00 {
            let mut buffer_trash: [u8; 512] = [0; 512];
            file.read(&mut buffer_trash).expect("Cannot read from cartridge file");
        }

        let n_mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1 >> 4);

        let n_prgbanks = header.prg_rom_chunks;
        let mut v_prg_memory = vec![0; (n_prgbanks as usize * 16384) as usize];
        file.read(&mut v_prg_memory).expect("Cannot read from cartridge file");

        let n_chrbanks = header.chr_rom_chunks;
        let mut v_chr_memory = vec![0; (n_chrbanks as usize * 8192) as usize];
        file.read(&mut v_chr_memory).expect("Cannot read from cartridge file");


        Cartridge {
            header,
            n_mapper_id,
            n_prgbanks,
            n_chrbanks,
            v_prg_memory,
            v_chr_memory
        }
    }
}