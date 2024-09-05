use rbca_core::Cpu;

#[test]
fn test_hello_world() {
    let mut cpu = Cpu::new();
    cpu.mem_bus.load_cart("../roms/hello-world.gb");
    println!("{}", cpu.mem_bus.cart.unwrap().header_info());
}
