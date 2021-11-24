use crate::chip8::keymap::KeyMap;
use crate::error::BuildManagerError;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const DISPLAY_W: u32 = 64;
const DISPLAY_H: u32 = 32;
const DISPLAY_SCALE: u32 = 20;

/// possible keyboard event
pub enum ManagerEvent {
    KeyDown(u8), // represent the event of pressing a key
    KeyUp(u8),   // represent the event of releasing a key
    Quit,        // quit event
    None,        // nothing happened
}

/// manage the video, audio and keyboard events
pub struct Manager {
    canvas: WindowCanvas,            // used to draw on the screen
    device: AudioDevice<SquareWave>, // used to handle the audio device
    event_pump: EventPump,           // used to listen for event
}

impl Manager {
    /// create a platform instance
    pub fn new() -> Result<Manager, BuildManagerError> {
        let w = DISPLAY_W * DISPLAY_SCALE;
        let h = DISPLAY_H * DISPLAY_SCALE;

        let sdl_context = sdl2::init().map_err(BuildManagerError::SdlContextError)?;
        let video_subsystem = sdl_context
            .video()
            .map_err(BuildManagerError::VideoSubsystemError)?;
        let audio_subsystem = sdl_context
            .audio()
            .map_err(BuildManagerError::AudioSubsystemError)?;

        let window = video_subsystem
            .window("CHIP-8", w, h)
            .position_centered()
            .build()
            .map_err(BuildManagerError::WindowError)?;

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
            .map_err(BuildManagerError::AudioDeviceError)?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(BuildManagerError::WindowCanvasError)?;

        let event_pump = sdl_context
            .event_pump()
            .map_err(BuildManagerError::EventPumpError)?;

        Ok(Manager {
            canvas,
            device,
            event_pump,
        })
    }

    /// resume or pause the audio device
    pub fn beep(&mut self, beep: bool) {
        if beep {
            self.device.resume();
        } else {
            self.device.pause();
        }
    }

    /// clear all pixels on the screen
    pub fn clear(&mut self) {
        self.canvas
            .set_draw_color(Color::RGBA(0x19, 0x14, 0x28, 0xFF));
        self.canvas.clear();
    }

    /// clear the current rendering target with the drawing color
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// update the screen with any rendering performed since the previous call
    pub fn draw_pixel(&mut self, x: u8, y: u8) -> Result<(), String> {
        self.canvas
            .set_draw_color(Color::RGBA(0xC8, 0xC8, 0xFF, 0xFF));

        let rect = Rect::new(
            DISPLAY_SCALE as i32 * x as i32,
            DISPLAY_SCALE as i32 * y as i32,
            DISPLAY_SCALE,
            DISPLAY_SCALE,
        );
        self.canvas.fill_rect(rect)?;
        Ok(())
    }

    /// listen for events(KeyDown, KeyUp, Quit, None)
    pub fn poll_event(&mut self, keymap: &KeyMap) -> ManagerEvent {
        if let Some(event) = self.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => ManagerEvent::Quit,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => ManagerEvent::Quit,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = keymap.keycode(keycode) {
                        ManagerEvent::KeyDown(key)
                    } else {
                        ManagerEvent::None
                    }
                }

                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = keymap.keycode(keycode) {
                        ManagerEvent::KeyUp(key)
                    } else {
                        ManagerEvent::None
                    }
                }

                _ => ManagerEvent::None,
            }
        } else {
            ManagerEvent::None
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
