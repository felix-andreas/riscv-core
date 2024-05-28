use crate::utils::{sign_extend, REGISTER_NAMES};

#[derive(Clone, Copy)]
pub struct RType(pub u32);
impl RType {
    pub fn rd(&self) -> u32 {
        self.0 >> 7 & 0b1_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
}

impl std::fmt::Debug for RType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} [R-Type]",
            REGISTER_NAMES[self.rd() as usize],
            REGISTER_NAMES[self.rs1() as usize],
            REGISTER_NAMES[self.rs2() as usize],
        )
    }
}

#[derive(Clone, Copy)]
pub struct IType(pub u32);
impl IType {
    pub fn rd(&self) -> u32 {
        self.0 >> 7 & 0b1_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn imm(&self) -> u32 {
        sign_extend(self.0 >> 20 & 0b1111_1111_1111, 11)
    }
}

impl std::fmt::Debug for IType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} 0x{:08x} [I-type]",
            REGISTER_NAMES[self.rd() as usize],
            REGISTER_NAMES[self.rs1() as usize],
            self.imm(),
        )
    }
}

#[derive(Clone, Copy)]
pub struct SType(pub u32);
impl SType {
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    pub fn imm(&self) -> u32 {
        sign_extend(
            (self.0 >> (25 - 5) & 0b1111_1110_0000) | (self.0 >> 7 & 0b0000_0001_1111),
            11,
        )
    }
}

impl std::fmt::Debug for SType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} 0x{:08x} [S-type]",
            REGISTER_NAMES[self.rs1() as usize],
            REGISTER_NAMES[self.rs2() as usize],
            self.imm(),
        )
    }
}

#[derive(Clone, Copy)]
pub struct BType(pub u32);
impl BType {
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    pub fn imm(&self) -> u32 {
        sign_extend(
            (self.0 >> (31 - 12) & 0b1_0000_0000_0000)
                | (self.0 >> (25 - 5) & 0b0_0111_1110_0000)
                | (self.0 >> (8 - 1) & 0b0_0000_0001_1110)
                | (self.0 << -(7 - 11) & 0b0_1000_0000_0000),
            12,
        )
    }
}

impl std::fmt::Debug for BType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} 0x{:08x} [B-type]",
            REGISTER_NAMES[self.rs1() as usize],
            REGISTER_NAMES[self.rs2() as usize],
            self.imm(),
        )
    }
}

#[derive(Clone, Copy)]
pub struct UType(pub u32);
impl UType {
    pub fn rd(&self) -> u32 {
        self.0 >> 7 & 0b1_1111
    }
    // TODO: sign extended immediate
    pub fn imm(&self) -> u32 {
        self.0 & 0b1111_1111_1111_1111_1111_0000_0000_0000
    }
}

impl std::fmt::Debug for UType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} 0x{:08x} [U-type]",
            REGISTER_NAMES[self.rd() as usize],
            self.imm()
        )
    }
}

#[derive(Clone, Copy)]
pub struct JType(pub u32);
impl JType {
    pub fn rd(&self) -> u32 {
        self.0 >> 7 & 0x1f
    }
    pub fn imm(&self) -> u32 {
        sign_extend(
            (self.0 >> (31 - 20) & 0b1_0000_0000_0000_0000_0000)
                | (self.0 & 0b0_1111_1111_0000_0000_0000)
                | (self.0 >> (20 - 11) & 0b0_0000_0000_1000_0000_0000)
                | (self.0 >> (21 - 1) & 0b0_0000_0111_1111_1110),
            20,
        )
    }
}

impl std::fmt::Debug for JType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} 0x{:08x} J-Type",
            REGISTER_NAMES[self.rd() as usize],
            self.imm()
        )
    }
}
