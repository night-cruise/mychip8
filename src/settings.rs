/// chip8 settings
pub struct Settings {
    pub cpu_freq: u16,         // cpu frequency
    pub delay_timer_freq: u16, // delay timer reduce frequency
    pub sound_timer_freq: u16, // sound timer reduce frequency

    // in some games, chip8 needs to increment the I register after executing LDI(FX55) and LDJ(FX65) instructions
    pub increment_i_register: bool,
    // in some games, chip8 needs to shift Vx and ignore Vy while executing SHR(8xy6) and SHL(8xyE) instructions
    pub shift_vx_ignore_vy: bool,
    // in some games, chip8 needs to set the Vf register when overflow occurs while executing ADDI(FX1E) instruction
    pub set_vf_when_overflow: bool,

    /// some games don't need sound
    pub mute: bool,

    // if the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen
    pub vertical_wrap: bool,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            cpu_freq: 700,
            delay_timer_freq: 60,
            sound_timer_freq: 60,
            increment_i_register: false,
            shift_vx_ignore_vy: true,
            set_vf_when_overflow: false,
            mute: false,
            vertical_wrap: false,
        }
    }
}
