use crate::chip8::{io, File, OpCode, Path, Read};

/// chip-8 has a 4KB memory
const MEMORY_SIZE: usize = 4096;

/// the start address of chip-8's sprites in memory
const SPRITE_ADDRESS: u16 = 0x0000;

/// chip-8 has a group of sprites representing the hexadecimal digits 0 though F
/// each sprite are 5 bytes long, or 8×5 model
const SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

/// chip-8 memory
pub struct Memory {
    // use an u8 array to emulate chip8 memory
    mem: [u8; MEMORY_SIZE],
}

impl Memory {
    /// create the memory instance
    pub fn new() -> Memory {
        let mut memory = [0; MEMORY_SIZE];

        // load digit sprites
        SPRITES.iter().enumerate().for_each(|(i, sprite)| {
            let address = Memory::sprite_address(i as u8) as usize;
            sprite
                .iter()
                .enumerate()
                .for_each(|(offset, &byte)| memory[address + offset] = byte);
        });

        Memory { mem: memory }
    }

    /// load the rom data to memory
    pub fn load_rom(&mut self, path: &Path) -> io::Result<()> {
        // read file to u8 vector
        let mut rom_data = vec![];
        File::open(path)?.read_to_end(&mut rom_data)?;

        // chip-8 programs start at location 0x200
        rom_data.iter().enumerate().for_each(|(i, &data)| {
            self.write(i as u16 + 0x200, data);
        });

        Ok(())
    }

    /// return the memory address of sprite
    pub fn sprite_address(sprite: u8) -> u16 {
        SPRITE_ADDRESS + (sprite as u16) * 5
    }

    /// read 1 byte data at address
    pub fn read8(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    /// read 2 bytes data at program counter
    pub fn read16(&self, address: u16) -> OpCode {
        OpCode::new(
            (self.mem[address as usize] as u16) << 8 | self.mem[(address as usize) + 1] as u16,
        )
    }

    /// write data to memory
    pub fn write(&mut self, address: u16, byte: u8) {
        self.mem[address as usize] = byte;
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new()
    }
}
