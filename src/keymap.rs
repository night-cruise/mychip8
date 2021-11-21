use sdl2::keyboard::Keycode;
// chip8 has a 16-key hexadecimal keypad with the following layout:
// 1  2  3  C
// 4  5  6  D
// 7  8  9  E
// A  0  B  F
// this layout should be mapped to today's platform to fit the keyboard

/// keyboard mapping
pub struct KeyMap;

impl KeyMap {
    /// create a keymap instance
    pub fn new() -> KeyMap {
        KeyMap
    }

    /// process keyboard mapping
    pub fn keycode(&self, key: Keycode) -> Option<u8> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }
}
