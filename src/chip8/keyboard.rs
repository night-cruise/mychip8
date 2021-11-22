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

    /// set keys
    // pub fn set_keys(&mut self, keys: [bool; 16]) {
    //     self.keys = keys;
    // }

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

    /// press the key
    pub fn press_key(&mut self, key: u8) {
        if self.key_press_wait {
            self.key_press = Some(key);
        }
        self.keys[key as usize] = true;
    }

    /// release the key
    pub fn release_key(&mut self, key: u8) {
        if let Some(k) = self.key_press {
            if k == key {
                self.key_press = None;
            }
        }
        self.keys[key as usize] = false;
    }
}

impl Default for KeyBoard {
    fn default() -> KeyBoard {
        KeyBoard::new()
    }
}
