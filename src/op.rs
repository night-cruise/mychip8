/// store opcode value
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
pub enum Op {
    LD,     // opcode: 6xkk, set Vx to kk
    ADD,    // opcode: 7xkk, add kk to Vx
    LDR,    // opcode: 8xy0, set Vx to Vy
}

impl Op {
    /// decode the type of opcode
    pub fn decode(opcode: &OpCode) -> Op {
        match opcode.get_opcode() & 0xF000 {
            0x6000 => Op::LD,
            0x7000 => Op::ADD,
            0x8000 => Op::LDR,
            _ => panic!("invalid opcode: {:04X}", opcode.get_opcode()),
        }
    }

    /// x in opcode 6xkk
    pub fn x(opcode: &OpCode) -> usize {
        ((opcode.get_opcode() & 0x0F00) >> 8) as usize
    }

    /// y in opcode 8xy0
    pub fn y(opcode: &OpCode) -> usize {
        ((opcode.get_opcode() & 0x00F0) >> 4) as usize
    }

    /// kk in opcode 7xkk
    pub fn kk(opcode: &OpCode) -> u8 {
        ((opcode.get_opcode() & 0x00FF)) as u8
    }
}