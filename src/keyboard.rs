/// chip8 keyboard
pub struct KeyBoard {
    keys: [bool; 16], // chip8 has a 16-key hexadecimal keypad
    key_press: Option<u8>,
    key_press_wait: bool,
}

impl KeyBoard {
    /// create a keyboard instance
    pub fn new() -> KeyBoard {
        KeyBoard {
            keys: [false; 16],
            key_press: None,
            key_press_wait: false,
        }
    }

    /// check whether the key is in the down position
    pub fn check_key(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    /// wait for a key press
    pub fn wait_key_press(&mut self) -> Option<u8> {
        if self.key_press_wait {
            if let Some(key) = self.key_press {
                self.key_press = None;
                self.key_press_wait = false;
                Some(key)
            } else {
                None
            }
        } else {
            self.key_press = None;
            self.key_press_wait = true;
            None
        }
    }
}
