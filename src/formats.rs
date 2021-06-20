pub struct RType(pub u32);
impl RType {
    pub fn rd(&self) -> u32 {
        self.0 >> 6 & 0b1_1111
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
            "R-type rd: {:<2} rs1: x{:<2} rs2: x{:<2}",
            self.rd(),
            self.rs1(),
            self.rs2(),
        )
    }
}

pub struct IType(pub u32);
impl IType {
    pub fn rd(&self) -> u32 {
        self.0 >> 6 & 0b1_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    // TODO: sign extended immediate
    pub fn imm(&self) -> u32 {
        self.0 >> 20 & 0b1111_1111_1111
    }
}

impl std::fmt::Debug for IType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "I-type rd: x{:<2}  rs1: x{:<2} imm: 0x{:05x}",
            self.rd(),
            self.rs1(),
            self.imm(),
        )
    }
}

pub struct SType(pub u32);
impl SType {
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    // TODO: sign extended immediate
    pub fn imm(&self) -> u32 {
        (self.0 >> (25 - 5) & 0b1111_1110_0000) | (self.0 >> 6 & 0b0000_0001_1111)
    }
}

impl std::fmt::Debug for SType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "S-type imm: rs1: x{:<2} rs2: x{:<2} 0x{:05x}",
            self.rs1(),
            self.rs2(),
            self.imm(),
        )
    }
}

pub struct BType(pub u32);
impl BType {
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    pub fn imm(&self) -> u32 {
        (self.0 >> (31 - 12) & 0b1_0000_0000_0000)
            | (self.0 >> (25 - 5) & 0b0_0111_1110_0000)
            | (self.0 >> (8 - 1) & 0b0_0000_0001_1110)
            | (self.0 << -(7 - 11) & 0b0_1000_0000_0000)
    }
}

impl std::fmt::Debug for BType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "B-type rs1: x{:<2} rs2: x{:<2} imm: 0x{:05x}",
            self.imm(),
            self.rs2(),
            self.rs1()
        )
    }
}

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
        write!(f, "U-type rd: x{:<2} imm: 0x{:05x}", self.rd(), self.imm())
    }
}

pub struct JType(pub u32);
impl JType {
    pub fn rd(&self) -> u32 {
        self.0 >> 6 & 0x1f
    }
    // TODO: sign extended immediate
    pub fn imm(&self) -> u32 {
        (self.0 >> (31 - 20) & 0b1_0000_0000_0000_0000_0000)
            | (self.0 & 0b0_1111_1111_0000_0000_0000)
            | (self.0 >> (20 - 11) & 0b0_0000_0000_1000_0000_0000)
            | (self.0 >> (21 - 1) & 0b0_0000_0111_1111_1110)
    }
}

impl std::fmt::Debug for JType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "J-type rd: x{:<2} imm: 0x{:05x}", self.rd(), self.imm())
    }
}
