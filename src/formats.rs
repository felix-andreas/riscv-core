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
