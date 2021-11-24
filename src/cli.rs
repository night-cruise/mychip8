use std::path::{Path, PathBuf};

use structopt::StructOpt;

/// A CHIP-8 game console emulator
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Prints the instructions
    #[structopt(short, long)]
    print_instruction: bool,

    /// game path
    #[structopt(name = "GAME_PATH", parse(from_os_str))]
    game_path: PathBuf,
}

impl Cli {
    /// check whether the game file existed
    pub fn is_game_file_existed(&self) -> bool {
        self.game_path.exists()
    }

    /// return the game file name
    pub fn game_name(&self) -> &str {
        self.game_path.file_name().unwrap().to_str().unwrap()
    }

    /// return the game file path
    pub fn game_path(&self) -> &Path {
        &self.game_path
    }

    /// check whether print the instructions
    pub fn if_print_instruction(&self) -> bool {
        self.print_instruction
    }
}
