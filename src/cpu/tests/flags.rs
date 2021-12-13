use crate::bus::Bus;
use crate::cpu::cpu_6502::Cpu;
use crate::cpu::Flags::{I, B, C, Z, V, N, D};

#[test]
fn test_flags() {
    let bus = Bus::new();
    let mut cpu = Cpu::new(bus);

    cpu.set_flag(C, true);
    assert_eq!(cpu.status, 1);
    cpu.set_flag(C, false);
    assert_eq!(cpu.status, 0);

    cpu.set_flag(Z, true);
    assert_eq!(cpu.status, 2);
    cpu.set_flag(Z, false);
    assert_eq!(cpu.status, 0);

    cpu.set_flag(I, true);
    assert_eq!(cpu.status, 4);
    cpu.set_flag(I, false);
    assert_eq!(cpu.status, 0);

    cpu.set_flag(D, true);
    assert_eq!(cpu.status, 8);
    cpu.set_flag(D, false);
    assert_eq!(cpu.status, 0);

    cpu.set_flag(B, true);
    assert_eq!(cpu.status, 16);
    cpu.set_flag(B, false);
    assert_eq!(cpu.status, 0);

    cpu.set_flag(V, true);
    assert_eq!(cpu.status, 64);
    cpu.set_flag(V, false);
    assert_eq!(cpu.status, 0);

    cpu.set_flag(N, true);
    assert_eq!(cpu.status, 128);
    cpu.set_flag(N, false);
    assert_eq!(cpu.status, 0);

}