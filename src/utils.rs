use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xmas_elf::program;

use crate::{Memory, Registers, MEMORY_START, PC};

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub fn sign_extend(number: u32, bit: u8) -> u32 {
    (number ^ (1 << bit)).overflowing_sub(1 << bit).0
}

pub fn load_word(memory: &Memory, address: u32) -> u32 {
    let address = address as usize - MEMORY_START;
    u32::from_le_bytes(memory[address..address + 4].try_into().unwrap())
}

pub fn store_word(memory: &mut Memory, address: u32, value: u32) {
    let address = address as usize - MEMORY_START;
    memory[address] = value as u8;
    memory[address + 1] = (value >> 8) as u8;
    memory[address + 2] = (value >> 16) as u8;
    memory[address + 3] = (value >> 24) as u8;
}

pub fn store_half_word(memory: &mut Memory, address: u32, value: u16) {
    let address = address as usize - MEMORY_START;
    memory[address] = value as u8;
    memory[address + 1] = (value >> 8) as u8;
}

pub fn store_byte(memory: &mut Memory, address: u32, value: u8) {
    let address = address as usize - MEMORY_START;
    memory[address] = value;
}

pub fn load_elf(memory: &mut Memory, path: &Path) {
    let mut buffer = Vec::new();
    {
        let mut file = File::open(path).unwrap();
        assert!(file.read_to_end(&mut buffer).unwrap() > 0);
    }

    let elf_file = xmas_elf::ElfFile::new(&buffer).unwrap();
    for program_header in elf_file.program_iter() {
        // TODO: revise this
        if program_header.physical_addr() == 0 {
            continue;
        }
        let address = program_header.physical_addr() as usize - MEMORY_START;
        if let Ok(program::SegmentData::Undefined(data)) = program_header.get_data(&elf_file) {
            memory[address..address + data.len()].copy_from_slice(data);
        } else {
            panic!("this should panic")
        }
    }
}

pub fn dump_registers(registers: &Registers) {
    let filler = "─".repeat(15);
    println!("╭{0:}┬{0:}┬{0:}┬{0:}╮", filler);
    println!(
        "│   pc {0:08x} │{1:}│{1:}│{1:}│",
        registers[PC],
        " ".repeat(15)
    );
    for i in 0..8 {
        for j in 0..4 {
            let index = i + j * 8;
            print!("│ {:>4} {:08x} ", REGISTER_NAMES[index], registers[index]);
        }
        println!("│");
    }
    println!("╰{0:}┴{0:}┴{0:}┴{0:}╯", filler);
}
