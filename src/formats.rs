#[derive(Debug)]
pub struct RType(pub u32);
#[allow(dead_code)]
impl RType {
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rd(&self) -> u32 {
        self.0 >> 6 & 0b1_1111
    }
}

#[derive(Debug)]
pub struct IType(pub u32);
#[allow(dead_code)]
impl IType {
    pub fn imm(&self) -> u32 {
        self.0 >> 20 & 0b1111_1111_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
    pub fn rd(&self) -> u32 {
        self.0 >> 6 & 0b1_1111
    }
}

#[derive(Debug)]
pub struct SType(pub u32);
#[allow(dead_code)]
impl SType {
    pub fn imm(&self) -> u32 {
        (self.0 >> (25 - 5) & 0b1111_1110_0000) | (self.0 >> 6 & 0b0000_0001_1111)
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
}

#[derive(Debug)]
pub struct BType(pub u32);
#[allow(dead_code)]
impl BType {
    pub fn imm(&self) -> u32 {
        (self.0 >> (31 - 12) & 0b1_0000_0000_0000)
            | (self.0 >> (25 - 5) & 0b0_0111_1110_0000)
            | (self.0 >> (8 - 1) & 0b0_0000_0001_1110)
            | (self.0 << -(7 - 11) & 0b0_1000_0000_0000)
    }
    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }
}

#[derive(Debug)]
pub struct UType(pub u32);
#[allow(dead_code)]
impl UType {
    fn imm(&self) -> u32 {
        self.0 & 0b1111_1111_1111_1111_1111_0000_0000_0000
    }
    fn rd(&self) -> u32 {
        self.0 >> 7 & 0b1_1111
    }
}

#[derive(Debug)]
pub struct JType(pub u32);
impl JType {
    pub fn imm(&self) -> u32 {
        (self.0 >> (31 - 20) & 0b1_0000_0000_0000_0000_0000)
            | (self.0 & 0b0_1111_1111_0000_0000_0000)
            | (self.0 >> (20 - 11) & 0b0_0000_0000_1000_0000_0000)
            | (self.0 >> (21 - 1) & 0b0_0000_0111_1111_1110)
    }
    pub fn rd(&self) -> u32 {
        self.0 >> 6 & 0x1f
    }
}
