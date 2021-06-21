use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xmas_elf::program;

use crate::{Memory, Registers, MEMORY_START, PC};

pub fn sign_extend(number: u32, bits: u8) -> u32 {
    (number ^ (1 << bits)).overflowing_sub(1 << bits).0
}

pub fn load_word(memory: &Memory, address: u32) -> u32 {
    let address = address as usize - MEMORY_START;
    u32::from_le_bytes(memory[address..address + 4].try_into().unwrap())
}

pub fn load_elf(memory: &mut Memory, path: &Path) {
    let mut buffer = Vec::new();
    {
        let mut file = File::open(path).unwrap();
        assert!(file.read_to_end(&mut buffer).unwrap() > 0);
    }

    let elf_file = xmas_elf::ElfFile::new(&&buffer).unwrap();
    for program_header in elf_file.program_iter() {
        let address = program_header.physical_addr() as usize - MEMORY_START;
        if let Ok(program::SegmentData::Undefined(data)) = program_header.get_data(&elf_file) {
            memory[address..address + data.len()].copy_from_slice(data);
        } else {
            panic!("this should panic")
        }
    }
}

pub fn dump_registers(registers: &Registers) {
    let filler = "─".repeat(14);
    println!("╭{0:}┬{0:}┬{0:}┬{0:}╮", filler);
    println!(
        "│ PC  {0:08x} │{1:}│{1:}│{1:}│",
        registers[PC],
        " ".repeat(14)
    );
    for i in 0..8 {
        for j in 0..4 {
            let index = i + j * 8;
            print!("│ x{:<02} {:08x} ", index, registers[index]);
        }
        println!("│");
    }
    println!("╰{0:}┴{0:}┴{0:}┴{0:}╯", filler);
}
