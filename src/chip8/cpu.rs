use crate::chip8::{Display, KeyBoard, Memory, Op, Rng, Settings};

/// chip-8 cpu
pub struct Cpu {
    v: [u8; 16], // general purpose 8-bit registers(from V0 to VF, and the VF is used as a flag by some instructions)
    i: u16,      // generally used to store memory address
    dt: u8,      // delay timer
    st: u8,      // sound timer
    pc: u16,     // store the currently executing address
    sp: u8,      // point to the topmost level of the stack
    stack: [u16; 16], // stack is an array of 16 16-bit values, used to store the address that the interpreter returns to when finished with a subroutine
}

impl Cpu {
    /// create the cpu instance
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200, // chip-8 programs start at location 0x200
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn cycle_dt(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn cycle_st(&mut self) -> bool {
        if self.st > 0 {
            self.st -= 1;
        }
        self.st > 0
    }

    /// fetch, decode and execute instruction
    pub fn pipeline_operation(
        &mut self,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &mut KeyBoard,
        settings: &Settings,
        print_instruction: bool,
    ) {
        // read 2 bytes opcode at program counter
        let opcode = memory.read16(self.pc);

        // increment the program counter
        self.pc += 2;

        // decode the instruction
        let op = Op::decode(&opcode);

        if print_instruction {
            println!("{:04X}: {:04X} {:?}", self.pc - 2, opcode.get_opcode(), op);
        }

        match op {
            Op::SYS { address } => self.sys(address),
            Op::CLS => self.cls(display),
            Op::RET => self.ret(),
            Op::JP { address } => self.jp(address),
            Op::CALL { address } => self.call(address),
            Op::SE { reg, byte } => self.se(reg, byte),
            Op::SNE2 { reg, byte } => self.sne2(reg, byte),
            Op::SEV { reg_x, reg_y } => self.sev(reg_x, reg_y),
            Op::LD { reg, byte } => self.ld(reg, byte),
            Op::ADD { reg, byte } => self.add(reg, byte),
            Op::LDR { reg_x, reg_y } => self.ldr(reg_x, reg_y),
            Op::OR { reg_x, reg_y } => self.or(reg_x, reg_y),
            Op::AND { reg_x, reg_y } => self.and(reg_x, reg_y),
            Op::XOR { reg_x, reg_y } => self.xor(reg_x, reg_y),
            Op::ADD2 { reg_x, reg_y } => self.add2(reg_x, reg_y),
            Op::SUB { reg_x, reg_y } => self.sub(reg_x, reg_y),
            Op::SHR { reg_x, reg_y } => self.shr(reg_x, reg_y, settings),
            Op::SUBN { reg_x, reg_y } => self.subn(reg_x, reg_y),
            Op::SHL { reg_x, reg_y } => self.shl(reg_x, reg_y, settings),
            Op::SNE { reg_x, reg_y } => self.sne(reg_x, reg_y),
            Op::LDA { address } => self.lda(address),
            Op::JPV { address } => self.jpv(address),
            Op::RND { reg, byte } => self.rnd(reg, byte),
            Op::DRW { reg_x, reg_y, n } => self.drw(reg_x, reg_y, n, memory, display, settings),
            Op::SKP { reg } => self.skp(reg, keyboard),
            Op::SKNP { reg } => self.sknp(reg, keyboard),
            Op::LDT { reg } => self.ldt(reg),
            Op::LDK { reg } => self.ldk(reg, keyboard),
            Op::LDF { reg } => self.ldf(reg),
            Op::LDS { reg } => self.lds(reg),
            Op::ADDI { reg } => self.addi(reg, settings),
            Op::LDX { reg } => self.ldx(reg),
            Op::LDB { reg } => self.ldb(reg, memory),
            Op::LDI { reg } => self.ldi(reg, memory, settings),
            Op::LDJ { reg } => self.ldj(reg, memory, settings),
        }
    }

    fn sys(&mut self, _address: u16) {
        // doesn't need to do this
    }

    fn cls(&mut self, display: &mut Display) {
        display.clear();
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jp(&mut self, address: u16) {
        self.pc = address;
    }

    fn call(&mut self, address: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = address;
    }

    fn se(&mut self, reg: u8, byte: u8) {
        self.pc += if self.v[reg as usize] == byte { 2 } else { 0 };
    }

    fn sne2(&mut self, reg: u8, byte: u8) {
        self.pc += if self.v[reg as usize] != byte { 2 } else { 0 };
    }

    fn sev(&mut self, reg_x: u8, reg_y: u8) {
        self.pc += if self.v[reg_x as usize] == self.v[reg_y as usize] {
            2
        } else {
            0
        };
    }

    fn ld(&mut self, reg: u8, byte: u8) {
        self.v[reg as usize] = byte;
    }

    fn add(&mut self, reg: u8, byte: u8) {
        let (result, _) = self.v[reg as usize].overflowing_add(byte);
        self.v[reg as usize] = result;
    }

    fn ldr(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] = self.v[reg_y as usize];
    }

    fn or(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] |= self.v[reg_y as usize];
    }

    fn and(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] &= self.v[reg_y as usize];
    }

    fn xor(&mut self, reg_x: u8, reg_y: u8) {
        self.v[reg_x as usize] ^= self.v[reg_y as usize];
    }

    fn add2(&mut self, reg_x: u8, reg_y: u8) {
        let (result, carry) = self.v[reg_x as usize].overflowing_add(self.v[reg_y as usize]);
        self.v[reg_x as usize] = result;
        self.v[0xF] = if carry { 1 } else { 0 };
    }

    fn sub(&mut self, reg_x: u8, reg_y: u8) {
        let (result, borrow) = self.v[reg_x as usize].overflowing_sub(self.v[reg_y as usize]);
        self.v[reg_x as usize] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    fn shr(&mut self, reg_x: u8, mut reg_y: u8, settings: &Settings) {
        if settings.shift_vx_ignore_vy {
            reg_y = reg_x;
        }
        self.v[0xF] = self.v[reg_y as usize] & 1;
        self.v[reg_x as usize] = self.v[reg_y as usize] >> 1;
    }

    fn subn(&mut self, reg_x: u8, reg_y: u8) {
        let (result, borrow) = self.v[reg_y as usize].overflowing_sub(self.v[reg_x as usize]);
        self.v[reg_x as usize] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    fn shl(&mut self, reg_x: u8, mut reg_y: u8, settings: &Settings) {
        if settings.shift_vx_ignore_vy {
            reg_y = reg_x;
        }
        self.v[0xF] = (self.v[reg_y as usize] >> 7) & 1;
        self.v[reg_x as usize] = self.v[reg_y as usize] << 1;
    }

    fn sne(&mut self, reg_x: u8, reg_y: u8) {
        self.pc += if self.v[reg_x as usize] != self.v[reg_y as usize] {
            2
        } else {
            0
        };
    }

    fn lda(&mut self, address: u16) {
        self.i = address;
    }

    fn jpv(&mut self, address: u16) {
        self.pc = self.v[0] as u16 + address;
    }

    fn rnd(&mut self, reg: u8, byte: u8) {
        self.v[reg as usize] = byte & rand::thread_rng().gen_range(0..=255);
    }

    fn drw(
        &mut self,
        reg_x: u8,
        reg_y: u8,
        n: u8,
        memory: &mut Memory,
        display: &mut Display,
        settings: &Settings,
    ) {
        let origin_x = self.v[reg_x as usize] as usize; // origin x coordinate
        let origin_y = self.v[reg_y as usize] as usize; // origin y coordinate
        let n = n as usize; // read n bytes

        let mut pixel_erased = false;

        // offset on the y coordinate
        for y_offset in 0..n {
            let y = origin_y + y_offset; // y coordinate
            if y >= Display::display_height() && !settings.vertical_wrap {
                break;
            }
            // read one byte
            let byte = memory.read8(self.i + y_offset as u16);

            // offset on the x coordinate
            (0..8).into_iter().for_each(|x_offset| {
                let x = origin_x + x_offset; // x coordinate
                let pixel = (byte >> (7 - x_offset) & 1) == 1; // get the pixel on the (x, y) coordinate
                pixel_erased |= display.set_pixel(x, y, pixel);
            });
        }

        self.v[0xF] = if pixel_erased { 1 } else { 0 };
    }

    fn skp(&mut self, reg: u8, keyboard: &KeyBoard) {
        self.pc += if keyboard.check_key(self.v[reg as usize]) {
            2
        } else {
            0
        };
    }

    fn sknp(&mut self, reg: u8, keyboard: &KeyBoard) {
        self.pc += if !keyboard.check_key(self.v[reg as usize]) {
            2
        } else {
            0
        };
    }

    fn ldt(&mut self, reg: u8) {
        self.v[reg as usize] = self.dt;
    }

    fn ldk(&mut self, reg: u8, keyboard: &mut KeyBoard) {
        if let Some(key) = keyboard.wait_key_press() {
            self.v[reg as usize] = key;
        } else {
            self.pc -= 2;
        }
    }

    fn ldf(&mut self, reg: u8) {
        self.dt = self.v[reg as usize];
    }

    fn lds(&mut self, reg: u8) {
        self.st = self.v[reg as usize];
    }

    fn addi(&mut self, reg: u8, settings: &Settings) {
        let (result, flag) = self.i.overflowing_add(self.v[reg as usize] as u16);
        self.i = result;

        if settings.set_vf_when_overflow {
            self.v[0xF] = if flag { 1 } else { 0 };
        }
    }

    fn ldx(&mut self, reg: u8) {
        self.i = Memory::sprite_address(self.v[reg as usize]);
    }

    fn ldb(&mut self, reg: u8, memory: &mut Memory) {
        memory.write(self.i, self.v[reg as usize] / 100);
        memory.write(self.i + 1, (self.v[reg as usize] / 10) % 10);
        memory.write(self.i + 2, self.v[reg as usize] % 10)
    }

    fn ldi(&mut self, reg: u8, memory: &mut Memory, settings: &Settings) {
        (0..=reg).into_iter().for_each(|i| {
            let address = self.i + i as u16;
            memory.write(address, self.v[i as usize])
        });

        if settings.increment_i_register {
            self.i += reg as u16 + 1;
        }
    }

    fn ldj(&mut self, reg: u8, memory: &Memory, settings: &Settings) {
        (0..=reg).into_iter().for_each(|i| {
            let address = self.i + i as u16;
            self.v[i as usize] = memory.read8(address);
        });

        if settings.increment_i_register {
            self.i += reg as u16 + 1;
        }
    }
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu::new()
    }
}
