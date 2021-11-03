use crate::mem::Memory;
use crate::op::Op;

pub struct CPU {
    v: [u8; 16],    // general purpose 8-bit registers(from V0 to VF, and the VF is used as a flag by some instructions)
    i: u16,         // generally used to store memory address
    dt: u8,         // delay timer
    st: u8,         // sound timer
    pc: u16,        // store the currently executing address
    sp: u8,         // point to the topmost level of the stack
}

impl CPU {
    /// create the cpu instance
    pub fn new() -> CPU {
        CPU {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,     // chip-8 programs start at location 0x200
            sp: 0
        }
    }

    /// fetch, decode and execute instruction cyclically
    pub fn cycle(&mut self, memory: &mut Memory) {
        // read 2 bytes opcode at program counter
        let opcode = memory.read16(self.pc);

        // increment the program counter
        self.pc += 2;

        // decode the instruction
        let op = Op::decode(&opcode);

        // parameter at different location
        let x = Op::x(&opcode);
        let y = Op::y(&opcode);
        let kk = Op::kk(&opcode);

        // execute the instruction
        match op {
            Op::LD => self.v[x] = kk,
            Op::ADD => self.v[x] += kk,
            Op::LDR => self.v[x] = self.v[y],
        }
    }

}