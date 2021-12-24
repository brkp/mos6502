use mos6502::*;

#[test]
fn opcode_0x98_implied_tya() {
    let mut cpu = cpu::CPU::new();
    cpu.bus.attach(0x0000, 0xffff, bus::MockRAM::new(0x10000)).unwrap();

    assert_eq!(2 + 2, 5);
}