use dmg_core::hardware;

fn run_emulator() -> Result<(), Box<dyn std::error::Error>> {
    println!("Create hardware assets...");
    let mut cpu = hardware::Cpu::new();

    println!("Load the boot ROM...");
    cpu.memory.load_file("docs/dmg_boot/dmg_boot.bin", 0)?;

    println!("Beginning execution...");
    loop {
        cpu.step();

        // Step over the video data
        if cpu.registers.read_reg16(hardware::registers::Reg16::PC) == 0xA8 {
            println!("SKIPPED VIDEO DATA");
            cpu.registers.write_reg16(
                hardware::registers::Reg16::PC,
                0xE0
            );
        }
    }
}

fn main() {
    if let Err(err) = run_emulator() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
