use dmg_core::hardware;

fn run_emulator() -> Result<(), Box<dyn std::error::Error>> {
    println!("Create hardware assets...");
    let mut cpu = hardware::Cpu::new(true);

    println!("Load the boot ROM...");
    cpu.memory.load_file("docs/dmg_boot/dmg_boot.bin", 0)?;

    println!("Beginning execution...");
    loop {
        cpu.step()?;
        cpu.memory.drain_log();
        cpu.dump_state();
        println!("");

        let halt_pc = 0xFFFF;
        if cpu.registers.read_reg16(hardware::registers::Reg16::PC) > halt_pc {
            println!("Hit HALT_PC 0x{:04x}, exiting", halt_pc);
            std::process::exit(0);
        }
    }
}

fn main() {
    if let Err(err) = run_emulator() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
