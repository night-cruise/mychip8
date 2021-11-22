use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::time::{Duration, Instant};

use rand::Rng;

use clock::Clock;
use cpu::Cpu;
use display::Display;
use keyboard::KeyBoard;
use keymap::KeyMap;
use memory::Memory;
use operation::{Op, OpCode};
use settings::Settings;

use crate::platform::{PlatForm, PlatformEvent};

pub mod clock;
pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod keymap;
pub mod memory;
pub mod operation;
pub mod settings;

const GAME_FILE: &str = "c8games/TICTAC";

/// the chip-8 interpreter
pub struct CHIP8 {
    cpu: Cpu,
    display: Display,
    memory: Memory,
    keyboard: KeyBoard,
    keymap: KeyMap,
    settings: Settings,
}

impl CHIP8 {
    /// create a chip-8 instance
    pub fn new() -> CHIP8 {
        CHIP8 {
            cpu: Cpu::default(),
            display: Display::default(),
            memory: Memory::default(),
            keyboard: KeyBoard::default(),
            keymap: KeyMap::default(),
            settings: Settings::default(),
        }
    }

    /// run chip-8 emulator
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.memory.load_rom(Path::new(GAME_FILE))?;

        let mut platform = PlatForm::new("CHIP-8")?;

        let mut cpu_clock = Clock::new(self.settings.cpu_freq);
        let mut dt_clock = Clock::new(self.settings.delay_timer_freq);
        let mut st_clock = Clock::new(self.settings.sound_timer_freq);

        let mut done = false;
        while !done {
            match platform.poll_event(&self.keymap) {
                PlatformEvent::KeyDown(key) => {
                    self.keyboard.press_key(key);
                }
                PlatformEvent::KeyUp(key) => self.keyboard.release_key(key),
                PlatformEvent::Quit => {
                    done = true;
                }
                PlatformEvent::None => {
                    if st_clock.tick() {
                        let beep = self.cpu.cycle_st();
                        platform.beep(beep && !self.settings.mute);
                    }

                    if dt_clock.tick() {
                        self.cpu.cycle_dt();
                    }

                    if cpu_clock.tick() {
                        self.cpu.pipeline_operation(
                            &mut self.memory,
                            &mut self.display,
                            &mut self.keyboard,
                            &self.settings,
                        );

                        if self.display.redraw() {
                            platform.clear();
                            self.display.draw(&mut platform)?;
                            platform.present();
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for CHIP8 {
    fn default() -> CHIP8 {
        CHIP8::new()
    }
}
