use std::env;

use rbca_core::Cpu;

// Example usage: cargo t info -- ../roms/zelda.gb --nocapture
#[test]
fn test_cart_info() {
    let args: Vec<_> = env::args().collect();

    let mut cpu = Cpu::new();
    for arg in &args[1..] {
        cpu.mem_bus.load_cart(arg);
        if cpu.mem_bus.cart().is_some() {
            println!("ping!");
            println!("{}", cpu.mem_bus.cart().unwrap().header_info());
        }
    }
}
