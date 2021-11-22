use crate::chip8::CHIP8;

pub mod chip8;
pub mod error;
pub mod platform;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut chip8 = CHIP8::default();
    chip8.run()?;

    Ok(())
}
