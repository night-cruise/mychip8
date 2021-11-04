use crate::mem::Memory;
use crate::op::Op;

pub struct CPU {
    v: [u8; 16],        // general purpose 8-bit registers(from V0 to VF, and the VF is used as a flag by some instructions)
    i: u16,             // generally used to store memory address
    dt: u8,             // delay timer
    st: u8,             // sound timer
    pc: u16,            // store the currently executing address
    sp: u8,             // point to the topmost level of the stack
    stack: [u16; 16]    // stack is an array of 16 16-bit values, used to store the address that the interpreter returns to when finished with a subroutine
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
            sp: 0,
            stack: [0; 16]
        }
    }

    /// fetch, decode and execute instruction
    pub fn cycle(&mut self, memory: &mut Memory) {
        // read 2 bytes opcode at program counter
        let opcode = memory.read16(self.pc);

        // increment the program counter
        self.pc += 2;

        // decode the instruction
        let op = Op::decode(&opcode);

        println!("{:04X}: {:04X}  {:?}", 0x200+self.pc, opcode.get_opcode(), op);
        match op {
            Op::SYS {address}           =>      self.sys(address),
            Op::CLS                          =>      self.cls(),
            Op::RET                          =>      self.ret(),
            Op::JP {address}            =>       self.jp(address),
            Op::CALL {address}          =>       self.call(address),
            Op::SE {reg, byte}      =>        self.se(reg, byte),
            Op::LD {reg, byte}      =>       self.ld(reg, byte),
            Op::ADD {reg, byte}     =>       self.add(reg, byte),
            Op::LDR {reg_x, reg_y}  =>       self.ldr(reg_x, reg_y),
            Op::OR {reg_x, reg_y}   =>       self.or(reg_x, reg_y),
            Op::AND {reg_x, reg_y}  =>       self.and(reg_x, reg_y),
            Op::XOR {reg_x ,reg_y}  =>       self.xor(reg_x, reg_y),
            Op::ADD2 {reg_x, reg_y} =>       self.add2(reg_x, reg_y),
            Op::SUB {reg_x, reg_y}  =>       self.sub(reg_x, reg_y),
            Op::SHR {reg_x, reg_y}  =>       self.shr(reg_x, reg_y),
            Op::SUBN {reg_x, reg_y} =>       self.subn(reg_x, reg_y),
            Op::SHL {reg_x, reg_y}  =>       self.shl(reg_x, reg_y),
            Op::LDA {address}           =>       self.lda(address),
            Op::LDI {reg}                =>       self.ldi(reg, memory),
            _ => todo!()
        }
    }

    fn sys(&mut self, _address: u16) {
        // doesn't need to do this
    }

    fn cls(&mut self) {
        todo!()
    }

    // the interpreter sets the program counter to the address at the top of the stack,
    // then subtracts 1 from the stack pointer.
    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jp(&mut self, address: u16) {
        self.pc = address;
    }

    // increments stack pointer, then puts the PC on the top of the stack. and set PC to nnn
    fn call(&mut self, address: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn se(&mut self, reg: u8, byte: u8) {
        if self.v[reg as usize] == byte {
            self.pc += 2;
        }
    }

    fn ld(&mut self, reg: u8, byte: u8) {
        self.v[reg as usize] = byte;
    }

    fn add(&mut self, reg: u8, byte: u8) {
        self.v[reg as usize] += byte;
    }

    fn ldr(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] = self.v[reg_y as usize];
    }

    fn or(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] = self.v[reg_x as usize] | self.v[reg_y as usize];
    }

    fn and(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] = self.v[reg_x as usize] & self.v[reg_y as usize];
    }

    fn xor(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] = self.v[reg_x as usize] ^ self.v[reg_y as usize];
    }

    // add Vx and Vy.
    // if the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    // only the lowest 8 bits of the result are kept, and stored in Vx.
    fn add2(&mut self, reg_x: u8, reg_y: u8) {
        let (result, carry) = self.v[reg_x as usize].overflowing_add(self.v[reg_y as usize]);
        self.v[reg_x as usize] = result;
        self.v[0xF] = if carry { 1 } else { 0 };
    }

    // if Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn sub(&mut self, reg_x: u8, reg_y: u8) {
        let (result, borrow) = self.v[reg_x as usize].overflowing_sub(self.v[reg_y as usize]);
        self.v[reg_x as usize] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    // if the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
    // then Vx is divided by 2.
    fn shr(&mut self, reg_x: u8, reg_y: u8) {
        self.v[0xF] = self.v[reg_x as usize] & 1;
        self.v[reg_x as usize] = self.v[reg_x as usize] >> 1;
    }

    // if Vy > Vx, then VF is set to 1, otherwise 0.
    // then Vx is subtracted from Vy, and the results stored in Vx.
    fn subn(&mut self, reg_x: u8, reg_y: u8) {
        let (result, borrow) = self.v[reg_y as usize].overflowing_sub(self.v[reg_x as usize]);
        self.v[reg_x as usize] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    // if the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
    // then Vx is multiplied by 2.
    fn shl(&mut self, reg_x: u8, reg_y: u8) {
        self.v[0xF] = if let 0b10000000 = self.v[reg_x as usize] & 0b10000000 {
            1
        } else {
            0
        };
        self.v[reg_x as usize] = self.v[reg_x as usize] << 1;
    }

    // if Vx != Vy, the program counter is increased by 2.
    fn sne(&mut self, reg_x: u8, reg_y: u8) {
        if self.v[reg_x as usize] != self.v[reg_y as usize] {
            self.pc += 2;
        }
    }

    fn lda(&mut self, address: u16) {
        self.i = address;
    }

    fn ldi(&mut self, reg: u8, memory: &mut Memory) {
        for i in 0..=reg {
            let address = (self.i + i as u16) as usize;
            memory.write(address, self.v[i as usize]);
        }
    }
}