use crate::chip8::CHIP8;
use crate::cli::Cli;

pub mod chip8;
pub mod cli;
pub mod error;
pub mod manager;

/// run the CHIP-8 emulator
pub fn run(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let mut chip8 = CHIP8::new(args.game_name());
    chip8.run(args.game_path(), args.if_print_instruction())?;

    Ok(())
}
