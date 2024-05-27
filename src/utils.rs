use std::convert::TryInto;

use crate::{Memory, Registers, MEMORY_START, PC};

pub const REGISTER_NAMES: [&str; 33] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6", "pc",
];

pub fn sign_extend(number: u32, bit: u8) -> u32 {
    (number ^ (1 << bit)).overflowing_sub(1 << bit).0
}

#[derive(Debug)]
pub struct MemoryError {
    pub address: u32,
}

pub fn load_word(memory: &Memory, address: u32) -> Result<u32, MemoryError> {
    let index = address as usize - MEMORY_START;
    Ok(u32::from_le_bytes(
        memory
            .get(index..index + 4)
            .ok_or(MemoryError { address })?
            .try_into()
            .unwrap(),
    ))
}

pub fn store_word(memory: &mut Memory, address: u32, value: u32) -> Result<(), MemoryError> {
    let index = address as usize - MEMORY_START;
    memory
        .get_mut(index..index + 4)
        .ok_or(MemoryError { address })?
        .copy_from_slice(&[
            value as u8,
            (value >> 8) as u8,
            (value >> 16) as u8,
            (value >> 24) as u8,
        ]);
    Ok(())
}

pub fn store_half_word(memory: &mut Memory, address: u32, value: u16) -> Result<(), MemoryError> {
    let index = address as usize - MEMORY_START;
    memory
        .get_mut(index..index + 2)
        .ok_or(MemoryError { address })?
        .copy_from_slice(&[value as u8, (value >> 8) as u8]);
    Ok(())
}

pub fn store_byte(memory: &mut Memory, address: u32, value: u8) -> Result<(), MemoryError> {
    let index = address as usize - MEMORY_START;
    *memory.get_mut(index).ok_or(MemoryError { address })? = value;
    Ok(())
}

pub fn dump_registers(registers: &Registers) -> String {
    let mut result = String::new();
    let filler = "─".repeat(15);
    result += &format!("╭{0:}┬{0:}┬{0:}┬{0:}╮\n", filler);
    result += &format!(
        "│   pc {0:08x} │{1:}│{1:}│{1:}│\n",
        registers[PC],
        " ".repeat(15)
    );
    for i in 0..8 {
        for j in 0..4 {
            let index = i + j * 8;
            result += &format!("│ {:>4} {:08x} ", REGISTER_NAMES[index], registers[index]);
        }
        result += "│\n";
    }
    result += &format!("╰{0:}┴{0:}┴{0:}┴{0:}╯\n", filler);
    result
}
