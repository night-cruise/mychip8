use std::fs::{self, File};
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("Hello World");
}


const MEMORY_SIZE: usize = 4096;

pub struct CHIP8 {
    /// use an u8 array to emulate chip8 memory
    mem: [u8; MEMORY_SIZE]
}

impl CHIP8 {
    /// create the chip8 instance
    pub fn new() -> CHIP8 {
        CHIP8 {
            mem: [0; MEMORY_SIZE]
        }
    }

    /// load the ROM into memory
    pub fn load_rom(&mut self, path: &Path) -> io::Result<()> {
        // Read file
        let rom_data = fs::read_to_string(path)?.as_bytes();

        // load the ROM into memory at 0x200
        // most chip-8 programs start at location 0x200
        // see http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1 for details
        for (i, &data) in rom_data.iter().enumerate() {
            self.mem[0x200+i] = data;
        }

        Ok(())
    }
}

pub struct CPU {
    v: [u8; 16],    // general purpose 8-bit registers(from V0 to VF, and the VF is used as a flag by some instructions)
    i: u16,         // generally used to store memory address
    dt: u8,         // delay timer
    st: u8,         // sound timer
    pc: u16,        // store the currently executing address
    sp: u8,         // point to the topmost level of the stack
}