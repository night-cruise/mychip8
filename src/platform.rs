use crate::keymap::KeyMap;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const DISPLAY_W: u32 = 64;
const DISPLAY_H: u32 = 32;
const DISPLAY_SCALE: u32 = 20;

pub enum PlatformEvent {
    KeyDown(u8),
    KeyUp(u8),
    Quit,
    None,
}

pub struct PlatForm {
    canvas: WindowCanvas,
    device: AudioDevice<SquareWave>,
    event_pump: EventPump,
}

impl PlatForm {
    pub fn new(name: &str) -> PlatForm {
        let w = DISPLAY_W * DISPLAY_SCALE;
        let h = DISPLAY_H * DISPLAY_SCALE;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        let window = video_subsystem
            .window(name, w, h)
            .position_centered()
            .build()
            .unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(523),
            channels: Some(1),
            samples: Some(32),
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        PlatForm {
            canvas,
            device,
            event_pump,
        }
    }

    pub fn keyboard_state(&mut self, keymap: &KeyMap) -> [bool; 16] {
        let state = self.event_pump.keyboard_state();

        let mut push_keys = [false; 16];
        for key in 0..0xF {
            if state.is_scancode_pressed(Scancode::from_keycode(keymap.keycode(key)).unwrap()) {
                push_keys[key as usize] = true;
            }
        }

        push_keys
    }

    pub fn beep(&mut self, beep: bool) {
        if beep {
            self.device.resume();
        } else {
            self.device.pause();
        }
    }

    pub fn clear(&mut self) {
        self.canvas
            .set_draw_color(Color::RGBA(0x19, 0x14, 0x28, 0xFF));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8) {
        self.canvas
            .set_draw_color(Color::RGBA(0xC8, 0xC8, 0xFF, 0xFF));

        let rect = Rect::new(
            DISPLAY_SCALE as i32 * x as i32,
            DISPLAY_SCALE as i32 * y as i32,
            DISPLAY_SCALE * 1,
            DISPLAY_SCALE * 1,
        );
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn poll_event(&mut self, keymap: &KeyMap) -> PlatformEvent {
        if let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => PlatformEvent::Quit,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let mut result = PlatformEvent::None;
                    for key in 0..0xF {
                        if keycode == keymap.keycode(key) {
                            result = PlatformEvent::KeyDown(key);
                            break;
                        }
                    }
                    result
                }

                Event::KeyUp {keycode: Some(keycode), ..} => {
                    let mut result = PlatformEvent::None;
                    for key in 0..0xF {
                        if keycode == keymap.keycode(key) {
                            result = PlatformEvent::KeyUp(key);
                            break;
                        }
                    }
                    result
                }

                Event::Quit { .. } => PlatformEvent::Quit,
                _ => PlatformEvent::None,
            }
        } else {
            PlatformEvent::None
        }
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}