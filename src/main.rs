use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xmas_elf::program;

mod formats;
mod instructions;

// use formats::{BType, IType, JType, RType, SType, UType};
use formats::JType;
use instructions::Instruction;

const PC: usize = 32;
const MEMORY_SIZE: usize = 0x10000;
const MEMORY_START: usize = 0x80000000;

type Register = [u32; 33];
type Memory = [u8; MEMORY_SIZE];

fn read_u32(memory: &Memory, address: u32) -> u32 {
    let address = address as usize - MEMORY_START;
    u32::from_le_bytes(memory[address..address + 4].try_into().unwrap())
}

fn load_elf(memory: &mut Memory, path: &Path) {
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

fn dump(registers: &Register) {
    for i in 0..4 {
        for j in 0..8 {
            let index = i * 8 + j;
            print!("x{:<2}: {:08x} ", index, registers[index]);
        }
        println!();
    }
    println!("PC : {:08x}\n", registers[PC]);
}

fn decode(code: u32) -> Instruction {
    let opcode = code & 0x7f;
    match opcode {
        // LUI
        0b0110111 => {
            unimplemented!("LUI")
        }
        // AUIPC
        0b0010111 => {
            unimplemented!("AUIPC");
        }
        // JAL
        0b1101111 => {
            println!("JAL");
            Instruction::JAL(JType(code))
        }
        // JALR
        0b1100111 => {
            unimplemented!("JALR");
        }
        // BRANCH
        0b1100011 => {
            unimplemented!("BRANCH");
        }
        // LOAD
        0b0000011 => {
            unimplemented!("LOAD");
        }
        // STORE
        0b0100011 => {
            unimplemented!("STORE");
        }
        // IMM
        0b0010011 => {
            unimplemented!("OP-IMM");
        }
        // OP
        0b0110011 => {
            unimplemented!("OP");
        }
        _ => {
            panic!("This instruction is not implemented yet, and I don't know what to do bye bye!");
        }
    }
}

fn step(registers: &mut Register, memory: &mut Memory) {
    // Instruction Decode
    let code = read_u32(&memory, registers[PC]);
    let instruction = decode(code);

    print!("instruction: {:?}", instruction);

    // Execute
    match instruction {
        Instruction::JAL(j_type) => {
            let rd = j_type.rd();
            let imm = j_type.imm();
            registers[PC] += imm;
            registers[rd as usize] = registers[PC] + 4;
        }
    }

    // Access
    // Write back
}

fn main() {
    let mut memory: Memory = [0; MEMORY_SIZE];
    let mut registers: Register = [0; 33];

    // for entry in glob::glob("riscv-tests/isa/rv32ui*").unwrap() {
    //     let path = entry.unwrap();
    //     if path.is_dir() || path.extension().is_some() {
    //         continue;
    //     }
    //
    // }

    let path = Path::new("riscv-tests/isa/rv32ui-v-add");

    println!("\nTest {:?}", path);
    load_elf(&mut memory, path);
    registers[PC] = MEMORY_START as u32;
    dump(&registers);

    for _ in 0..3 {
        step(&mut registers, &mut memory);
        dump(&registers);
    }
}
