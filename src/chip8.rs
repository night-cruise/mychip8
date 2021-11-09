use crate::cpu::CPU;
use crate::display::Display;
use crate::keyboard::KeyBoard;
use crate::memory::Memory;
use std::path::Path;
use crate::settings::Settings;

const GAME_FILE: &str = "c8games/BLINKY";

/// the chip-8 interpreter
pub struct CHIP8;

impl CHIP8 {
    /// run chip-8 emulator
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut memory = Memory::new();
        memory.load_rom(Path::new(GAME_FILE))?;

        let mut display = Display::new();

        let mut keyboard = KeyBoard::new();

        let mut cpu = CPU::new();

        let settings = Settings::new();

        for _i in 0..500 {
            cpu.cycle(&mut memory, &mut display, &mut keyboard, &settings);
        }

        Ok(())
    }
}
