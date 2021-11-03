use std::{fs, io};
use std::io::Read;
use std::path::Path;
use crate::op::OpCode;

const MEMORY_SIZE: usize = 4096;

/// chip-8 memory
pub struct Memory {
    // use an u8 array to emulate chip8 memory
    mem: [u8; MEMORY_SIZE]
}

impl Memory {
    /// create the memory instance
    pub fn new() -> Memory {
        Memory {
            mem: [0; MEMORY_SIZE]
        }
    }

    /// load the rom data to memory
    pub fn load_rom(&mut self, path: &Path) -> io::Result<()> {
        // open file
        let mut rom_file = fs::File::open(path)?;

        // read file data to u8 vector
        let mut rom_data = vec![];
        rom_file.read_to_end(&mut rom_data)?;

        // most chip-8 programs start at location 0x200
        // see http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1 for details
        for (index, &data) in rom_data.iter().enumerate() {
            self.mem[0x200 + index] = data;
        }

        Ok(())
    }

    /// read 2 bytes data at program counter
    pub fn read16(&self, pc: u16) -> OpCode {
        let pc = pc as usize;
        OpCode::new(self.mem[pc..(pc+2)].as_ptr() as u16)
    }
}