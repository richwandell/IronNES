use crate::cartridge::Cartridge;

#[test]
fn test_read_cartridge_header() {
    let cart = Cartridge::new("assets/nestest.nes");

    println!("{}", "hi");
}