pub fn sign_extend(number: u32, bits: u8) -> u32 {
    (number ^ (1 << bits)).overflowing_sub(1 << bits).0
}
