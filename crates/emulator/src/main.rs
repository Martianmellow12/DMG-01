use dmg_core::{hardware, software};

fn run_emulator() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading ROM...");
    let rom_path = "roms/super_mario_land_rev1.gb";
    let gamerom = software::gamerom::GameRom::new(rom_path)?;
    
    println!("Create hardware assets...");
    let mut cpu = hardware::cpu::Cpu::new(gamerom, true);

    println!("Load the boot ROM...");
    let brom_vec = std::fs::read("docs/dmg_boot/dmg_boot.bin")?;
    let brom_bytes: [u8; 256] = brom_vec.try_into().expect("BROM isn't the correct format");
    cpu.bus.load_brom(brom_bytes);

    println!("Beginning execution...");
    loop {
        cpu.step()?;
        //cpu.bus.drain_log();
        cpu.dump_state();
        println!("");

        let halt_pc = 0xFFFF;
        if cpu.registers.read_reg16(hardware::cpu::registers::Reg16::PC) > halt_pc {
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
