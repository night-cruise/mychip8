use std::path::Path;
use crate::cpu::CPU;
use crate::mem::Memory;

const GAME_FILE: &str = "c8games/BLINKY";

/// the chip-8 interpreter
pub struct CHIP8;

impl CHIP8 {
    /// run chip-8 emulator
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut memory = Memory::new();
        memory.load_rom(Path::new(GAME_FILE))?;

        let mut cpu = CPU::new();

        cpu.cycle(&mut memory);

        Ok(())
    }
}