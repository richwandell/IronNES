use crate::cartridge::Cartridge;

#[test]
fn test_read_cartridge_header() {
    let cart = Cartridge::new("junk/nes-test-roms-master/other/nestest.nes");

    println!("{}", "hi");
}