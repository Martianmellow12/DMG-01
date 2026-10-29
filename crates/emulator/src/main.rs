use dmg_core::hardware;

fn run_emulator() -> Result<(), Box<dyn std::error::Error>> {
    println!("Create hardware assets...");
    let mut cpu = hardware::Cpu::new();

    println!("Load the boot ROM...");
    cpu.memory.load_file("docs/dmg_boot/dmg_boot.bin", 0)?;

    println!("Beginning execution...");
    loop {
        cpu.step();
        cpu.dump_state();
    }
}

fn main() {
    if let Err(err) = run_emulator() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
