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

use crate::manager::{Manager, ManagerEvent};

pub mod clock;
pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod keymap;
pub mod memory;
pub mod operation;
pub mod settings;

/// the chip-8 interpreter
pub struct CHIP8 {
    cpu: Cpu,
    display: Display,
    memory: Memory,
    keyboard: KeyBoard,
    keymap: KeyMap,
    settings: Settings,
    cpu_clock: Clock,
    dt_clock: Clock,
    st_clock: Clock,
}

impl CHIP8 {
    /// create a chip-8 instance
    pub fn new(game_name: &str) -> CHIP8 {
        let cpu = Cpu::default();
        let display = Display::default();
        let memory = Memory::default();
        let keyboard = KeyBoard::default();
        let keymap = KeyMap::default();
        let settings = Settings::new(game_name);
        let cpu_clock = Clock::new(settings.cpu_freq);
        let st_clock = Clock::new(settings.sound_timer_freq);
        let dt_clock = Clock::new(settings.delay_timer_freq);
        CHIP8 {
            cpu,
            display,
            memory,
            keyboard,
            keymap,
            settings,
            cpu_clock,
            st_clock,
            dt_clock,
        }
    }

    /// run chip-8 emulator
    pub fn run(
        &mut self,
        game_path: &Path,
        print_instruction: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.memory.load_rom(game_path)?;
        println!("INFO: Game was loaded to memory successfully.");
        println!("INFO: Start running the CHIP-8 emulator.");

        let mut manager = Manager::new()?;

        loop {
            match manager.poll_event(&self.keymap) {
                ManagerEvent::KeyDown(key) => {
                    self.keyboard.press_key(key);
                }
                ManagerEvent::KeyUp(key) => self.keyboard.release_key(key),
                ManagerEvent::Quit => {
                    break;
                }
                ManagerEvent::None => {
                    if self.st_clock.tick() {
                        let beep = self.cpu.cycle_st();
                        manager.beep(beep && !self.settings.mute);
                    }

                    if self.dt_clock.tick() {
                        self.cpu.cycle_dt();
                    }

                    if self.cpu_clock.tick() {
                        self.cpu.pipeline_operation(
                            &mut self.memory,
                            &mut self.display,
                            &mut self.keyboard,
                            &self.settings,
                            print_instruction,
                        );

                        if self.display.redraw() {
                            manager.clear();
                            self.display.draw(&mut manager)?;
                            manager.present();
                        }
                    }
                }
            }
        }

        println!("INFO: Game over.");
        println!("INFO: Exist the CHIP-8 emulator.");
        Ok(())
    }
}
