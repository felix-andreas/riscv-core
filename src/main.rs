use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xmas_elf::program;

mod formats;
mod instructions;

use formats::{BType, IType, JType, RType, SType, UType};
use instructions::Instruction;

const PC: usize = 32;
const MEMORY_SIZE: usize = 0x10000;
const MEMORY_START: usize = 0x80000000;

type Registers = [u32; 33];
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

fn dump_registers(registers: &Registers) {
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

fn decode(code: u32) -> Instruction {
    let opcode = code & 0b111_1111;
    let funct3 = code >> 12 & 0b111;
    let funct7 = code >> 25 & 0b111_1111;
    match opcode {
        // LUI
        0b0110111 => Instruction::LUI(UType(code)),
        // AUIPC
        0b0010111 => Instruction::AUIPC(UType(code)),
        // JAL
        0b1101111 => Instruction::JAL(JType(code)),
        // JALR
        0b1100111 => Instruction::JALR(IType(code)),
        // BRANCH
        0b1100011 => match funct3 {
            0b000 => Instruction::BEQ(BType(code)),
            0b001 => Instruction::BNE(BType(code)),
            0b100 => Instruction::BLT(BType(code)),
            0b101 => Instruction::BGE(BType(code)),
            0b110 => Instruction::BLTU(BType(code)),
            0b111 => Instruction::BGEU(BType(code)),
            _ => unreachable!(),
        },
        // LOAD
        0b0000011 => match funct3 {
            0b000 => Instruction::LB(IType(code)),
            0b001 => Instruction::LH(IType(code)),
            0b010 => Instruction::LW(IType(code)),
            0b100 => Instruction::LBU(IType(code)),
            0b101 => Instruction::LHU(IType(code)),
            _ => unreachable!(),
        },
        // STORE
        0b0100011 => match funct3 {
            0b000 => Instruction::SB(SType(code)),
            0b001 => Instruction::SH(SType(code)),
            0b010 => Instruction::SW(SType(code)),
            _ => unreachable!(),
        },
        // OP-IMM
        0b0010011 => match funct3 {
            0b000 => Instruction::ADDI(IType(code)),
            0b010 => Instruction::SLTI(IType(code)),
            0b011 => Instruction::SLTIU(IType(code)),
            0b100 => Instruction::XORI(IType(code)),
            0b110 => Instruction::ORI(IType(code)),
            0b111 => Instruction::ANDI(IType(code)),
            0b001 => Instruction::SLLI(IType(code)),
            0b101 => match funct7 {
                0b0000000 => Instruction::SRLI(IType(code)),
                0b0100000 => Instruction::SRAI(IType(code)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        // OP
        0b0110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => Instruction::ADD(RType(code)),
                0b0100000 => Instruction::SUB(RType(code)),
                _ => unreachable!(),
            },
            0b001 => Instruction::SLL(RType(code)),
            0b010 => Instruction::SLT(RType(code)),
            0b011 => Instruction::SLTU(RType(code)),
            0b100 => Instruction::XOR(RType(code)),
            0b101 => match funct7 {
                0b0000000 => Instruction::SRL(RType(code)),
                0b0100000 => Instruction::SRA(RType(code)),
                _ => unreachable!(),
            },
            0b110 => Instruction::OR(RType(code)),
            0b111 => Instruction::AND(RType(code)),
            _ => unreachable!(),
        },
        // FENCE
        0b0001111 => Instruction::FENCE,
        // SYSTEM
        0b1110011 => match code >> 20 & 1 {
            0 => Instruction::ECALL,
            1 => Instruction::EBREAK,
            _ => unreachable!(),
        },
        _ => panic!("Don't know how to decode this :("),
    }
}

fn step(registers: &mut Registers, memory: &mut Memory) {
    // Instruction Fetch
    let pc = registers[PC];
    let mut next_pc = pc + 4;
    let code = read_u32(&memory, pc);

    // Instruction Decode
    let instruction = decode(code);
    println!("{:08x?}", &instruction);

    // Execute
    let rd: Option<u32>;
    let rd_value;
    match instruction {
        // LUI
        Instruction::LUI(u_type) => {
            rd = Some(u_type.rd());
            rd_value = u_type.imm();
        }
        // AUIPC
        Instruction::AUIPC(u_type) => {
            rd = Some(u_type.rd());
            rd_value = pc + u_type.imm();
        }
        // JAL
        Instruction::JAL(j_type) => {
            next_pc = pc + j_type.imm();
            rd = Some(j_type.rd());
            rd_value = pc + 4;
        }
        // OP-IMM // TODO: use sign extended values
        Instruction::ADDI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = (i_type.imm() as i32 + i_type.rs1() as i32) as u32;
        }
        Instruction::SLTI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = ((i_type.rs1() as i32) < (i_type.imm() as i32)) as u32;
        }
        Instruction::SLTIU(i_type) => {
            rd = Some(i_type.rd());
            rd_value = (i_type.rs1() < i_type.imm()) as u32;
        }
        _ => {
            println!("instruction: {:?}", &instruction);
            panic!("This instruction is not implemented yet, and I don't know what to do bye bye!")
        }
    }

    // Memory Access

    // Register Write Back
    registers[PC] = next_pc;
    if let Some(register) = rd {
        registers[register as usize] = rd_value
    }
}

fn main() {
    let mut memory: Memory = [0; MEMORY_SIZE];
    let mut registers: Registers = [0; 33];

    // for entry in glob::glob("riscv-tests/isa/rv32ui*").unwrap() {
    //     let path = entry.unwrap();
    //     if path.is_dir() || path.extension().is_some() {
    //         continue;
    //     }
    //
    // }

    let path = Path::new("riscv-tests/isa/rv32ui-v-add");

    println!("\nStart of: {:?}", path);
    load_elf(&mut memory, path);
    registers[PC] = MEMORY_START as u32;

    for i in 0..50 {
        print!("{:4} {:4x} ", i, registers[PC]);
        step(&mut registers, &mut memory);
        if i == 32 {
            dump_registers(&registers);
        }
    }
}
