/// store opcode value
#[derive(Debug)]
pub struct OpCode(u16);

impl OpCode {
    /// create an opcode instance
    pub fn new(opcode: u16) -> OpCode {
        OpCode(opcode)
    }

    /// get the opcode value
    pub fn get_opcode(&self) -> u16 {
        self.0
    }
}

/// opcode type
#[derive(Debug)]
pub enum Op {
    SYS {address: u16},                 // opcode: 0nnn, jump to a machine code routine at nnn
    CLS,                                // opcode: 00E0, clear the display (todo!)
    RET,                                // opcode: 00EE, return from a subroutine
    JP  {address: u16},                 // opcode: 1nnn, set PC to nnn
    CALL{address: u16},                 // opcode: 2nnn, call subroutine at nnn
    SE  {reg: u8, byte: u8},            // opcode: 3xkk, skip next instruction if Vx = kk
    LD  {reg: u8, byte: u8},            // opcode: 6xkk, set Vx to kk
    ADD {reg: u8, byte: u8},            // opcode: 7xkk, add kk to Vx
    LDR {reg_x: u8, reg_y: u8},         // opcode: 8xy0, set Vx to Vy
    OR  {reg_x: u8, reg_y: u8},         // opcode: 8xy1, set Vx = Vx OR Vy
    AND {reg_x: u8, reg_y: u8},         // opcode: 8xy2, set Vx = Vx AND Vy
    XOR {reg_x: u8, reg_y: u8},         // opcode: 8xy3, set Vx = Vx XOR Vy
    ADD2{reg_x: u8, reg_y: u8},         // opcode: 8xy4, set Vx = Vx + Vy, set VF = carry
    SUB {reg_x: u8, reg_y: u8},         // opcode: 8xy5, set Vx = Vx - Vy, set VF = NOT borrow
    SHR {reg_x: u8, reg_y: u8},         // opcode: 8xy6, set Vx = Vx SHR 1 (todo!)
    SUBN{reg_x: u8, reg_y: u8},         // opcode: 8xy7, set Vx = Vy - Vx, set VF = NOt borrow
    SHL {reg_x: u8, reg_y: u8},         // opcode: 8xyE, set Vx = Vx SHL 1 (todo!)
    SNE {reg_x: u8, reg_y: u8},         // opcode: 9xy0, skip next instruction if Vx != Vy.
    LDA {address: u16},                 // opcode: Annn, set I to nnn
    JPV {address: u16},                 // opcode: Bnnn, jump to location nnn + V0
    RND {reg_x: u8, byte: u8},          // opcode: Cxkk, set Vx = random byte AND kk
    DRW {reg_x: u8, reg_y: u8, n: u8},  // opcode: Dxyn, display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
    SKP {reg_x: u8},                    // opcode: Ex9E, skip next instruction if key with the value of Vx is pressed
    SKNP{reg_x: u8},                    // opcode: ExA1, skip next instruction if key with the value of Vx is not pressed
    LDI {reg: u8},                      // opcode: Fx55, store registers V0 through Vx in memory starting at location I.
}

impl Op {
    /// decode the type of opcode
    pub fn decode(opcode: &OpCode) -> Op {
        let opcode = opcode.get_opcode();
        match opcode & 0xF000 {
            0x0000 => {
              match opcode & 0x0FFF {
                  0x00E0 => Op::cls(),
                  0x00EE => Op::ret(),
                  _ => Op::sys(opcode)
              }
            },
            0x1000 => Op::jp(opcode),
            0x2000 => Op::call(opcode),
            0x3000 => Op::se(opcode),
            0x6000 => Op::ld(opcode),
            0x7000 => Op::add(opcode),
            0x8000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        Op::ldr(opcode)
                    },
                    0x0001 => {
                        Op::or(opcode)
                    },
                    0x0002 => {
                        Op::and(opcode)
                    },
                    0x0003 => {
                        Op::xor(opcode)
                    },
                    0x0004 => {
                        Op::add2(opcode)
                    },
                    0x0005 => {
                        Op::sub(opcode)
                    },
                    0x0006 => {
                        Op::shr(opcode)
                    },
                    0x0007 => {
                        Op::subn(opcode)
                    },
                    0x000E => {
                        Op::shl(opcode)
                    },
                    _ => panic!("invalid opcode: {:04X}", opcode),
                }
            },
            0x9000 => Op::sne(opcode),
            0xA000 => Op::lda(opcode),
            0xB000 => Op::jpv(opcode),
            0xC000 => Op::rnd(opcode),
            0xD000 => Op::drw(opcode),
            0xE000 => {
              match opcode & 0x00FF {
                  0x009E => Op::skp(opcode),
                  0x00A1 => Op::sknp(opcode),
                  _ => panic!("invalid opcode: {:04X}", opcode)
              }
            },
            0xF000 => Op::ldi(opcode),
            _ => panic!("invalid opcode: {:04X}", opcode),
        }
    }

    fn sys(opcode: u16) -> Op {
        Op::SYS {
            address: opcode & 0x0FFF
        }
    }

    fn cls() -> Op {
        Op::CLS
    }

    fn ret() -> Op {
        Op::RET
    }

    fn jp(opcode: u16) -> Op {
        Op::JP {
            address: opcode & 0x0FFF
        }
    }

    fn call(opcode: u16) -> Op {
        Op::CALL {
            address: opcode & 0x0FFF
        }
    }

    fn se(opcode: u16) -> Op {
        Op::SE {
            reg: ((opcode & 0x0F00) >> 8) as u8,
            byte: (opcode & 0x00FF) as u8
        }
    }

    fn ld(opcode: u16) -> Op {
        Op::LD {
            reg: ((opcode & 0x0F00) >> 8) as u8,
            byte: (opcode & 0x00FF) as u8
        }
    }

    fn add(opcode: u16) -> Op {
        Op::ADD {
            reg: ((opcode & 0x0F00) >> 8) as u8,
            byte: (opcode & 0x00FF) as u8
        }
    }

    fn ldr(opcode: u16) -> Op {
        Op::LDR {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8
        }
    }

    fn or(opcode: u16) -> Op {
        Op::OR {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn and(opcode: u16) -> Op {
        Op::AND {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn xor(opcode: u16) -> Op {
        Op::XOR {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn add2(opcode: u16) -> Op {
        Op::ADD2 {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn sub(opcode: u16) -> Op {
        Op::SUB {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn shr(opcode: u16) -> Op {
        Op::SHR {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn subn(opcode: u16) -> Op {
        Op::SUBN {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn shl(opcode: u16) -> Op {
        Op::SHL {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn sne(opcode: u16) -> Op {
        Op::SNE {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u8,
        }
    }

    fn lda(opcode: u16) -> Op {
        Op::LDA {
            address: opcode & 0x0FFF
        }
    }

    fn jpv(opcode: u16) -> Op {
        Op::JPV {
            address: opcode & 0x0FFF
        }
    }

    fn rnd(opcode: u16) -> Op {
        Op::RND {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            byte: (opcode & 0x00FF) as u8
        }
    }

    fn drw(opcode: u16) -> Op {
        Op::DRW {
            reg_x: ((opcode & 0x0F00) >> 8) as u8,
            reg_y: ((opcode & 0x00F0) >> 4) as u7,
            n: (opcode & 0x000F) as u8
        }
    }

    fn skp(opcode: u16) -> Op {
        Op::SKP {
            reg_x: ((opcode & 0x0F00) >> 8) as u8
        }
    }

    fn sknp(opcode: u16) -> Op {
        Op::SKNP {
            reg_x: ((opcode & 0x0F00) >> 8) as u8
        }
    }

    fn ldi(opcode: u16) -> Op {
        Op::LDI {
            reg: ((opcode & 0x0F00) >> 8) as u8
        }
    }
}