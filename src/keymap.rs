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
    pub fn keycode(&self, key: u8) -> Keycode {
        match key {
            0x1 => Keycode::Num1,
            0x2 => Keycode::Num2,
            0x3 => Keycode::Num3,
            0xC => Keycode::Num4,
            0x4 => Keycode::Q,
            0x5 => Keycode::W,
            0x6 => Keycode::E,
            0xD => Keycode::R,
            0x7 => Keycode::A,
            0x8 => Keycode::S,
            0x9 => Keycode::D,
            0xE => Keycode::F,
            0xA => Keycode::Z,
            0x0 => Keycode::X,
            0xB => Keycode::C,
            0xF => Keycode::V,
            _ => panic!("key out of range"),
        }
    }
}
