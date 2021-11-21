use crate::clock::Clock;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::keyboard::KeyBoard;
use crate::keymap::KeyMap;
use crate::memory::Memory;
use crate::platform::{PlatForm, PlatformEvent};
use crate::settings::Settings;
use std::path::Path;

const GAME_FILE: &str = "c8games/TICTAC";

/// the chip-8 interpreter
pub struct CHIP8;

impl CHIP8 {
    /// run chip-8 emulator
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut memory = Memory::new();
        memory.load_rom(Path::new(GAME_FILE))?;

        let mut display = Display::new();

        let mut keyboard = KeyBoard::new();

        let mut cpu = Cpu::new();

        let settings = Settings::new();

        let keymap = KeyMap::new();

        let mut platform = PlatForm::new("CHIP-8")?;

        let mut cpu_clock = Clock::new(settings.cpu_freq);
        let mut dt_clock = Clock::new(settings.delay_timer_freq);
        let mut st_clock = Clock::new(settings.sound_timer_freq);

        let mut done = false;
        while !done {
            match platform.poll_event(&keymap) {
                PlatformEvent::KeyDown(key) => {
                    keyboard.press_key(key);
                }
                PlatformEvent::KeyUp(key) => keyboard.release_key(key),
                PlatformEvent::Quit => {
                    done = true;
                }
                PlatformEvent::None => {
                    if st_clock.tick() {
                        let beep = cpu.cycle_st();
                        platform.beep(beep && !settings.mute);
                    }

                    if dt_clock.tick() {
                        cpu.cycle_dt();
                    }

                    if cpu_clock.tick() {
                        cpu.pipeline_operation(&mut memory, &mut display, &mut keyboard, &settings);

                        if display.redraw() {
                            platform.clear();
                            display.draw(&mut platform)?;
                            platform.present();
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
