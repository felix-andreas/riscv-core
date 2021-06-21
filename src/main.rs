mod formats;
mod instructions;
mod utils;

use formats::{BType, IType, JType, RType, SType, UType};
use instructions::Instruction;
use utils::{dump_registers, load_elf, sign_extend};
use utils::{load_word, store_byte, store_half_word, store_word};

const PC: usize = 32;
const MEMORY_SIZE: usize = 0x10000;
const MEMORY_START: usize = 0x80000000;

type Registers = [u32; 33];
type Memory = [u8; MEMORY_SIZE];

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
    let code = load_word(memory, pc);
    print!("{:08x} ", code);

    // Instruction Decode
    let instruction = decode(code);
    println!("{:08x?}", &instruction);

    // Execute
    let mut rd: Option<u32> = None;
    let mut rd_value = 0;
    match instruction {
        // LUI
        Instruction::LUI(u_type) => {
            rd = Some(u_type.rd());
            rd_value = u_type.imm();
        }
        // AUIPC
        Instruction::AUIPC(u_type) => {
            rd = Some(u_type.rd());
            rd_value = pc.overflowing_add(u_type.imm()).0;
        }
        // JAL
        Instruction::JAL(j_type) => {
            next_pc = pc.overflowing_add(j_type.imm()).0;
            rd = Some(j_type.rd());
            rd_value = pc + 4;
        }
        // JALR
        Instruction::JALR(i_type) => {
            next_pc = (registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0)
                & !1;
            rd = Some(i_type.rd());
            rd_value = pc + 4;
        }
        // BRANCH
        Instruction::BEQ(b_type) => {
            if registers[b_type.rs1() as usize] == registers[b_type.rs2() as usize] {
                next_pc = pc.overflowing_add(b_type.imm()).0;
            }
        }
        Instruction::BNE(b_type) => {
            if registers[b_type.rs1() as usize] != registers[b_type.rs2() as usize] {
                next_pc = pc.overflowing_add(b_type.imm()).0;
            }
        }
        Instruction::BLT(b_type) => {
            if (registers[b_type.rs1() as usize] as i32) < (registers[b_type.rs2() as usize] as i32)
            {
                next_pc = pc.overflowing_add(b_type.imm()).0;
            }
        }
        Instruction::BGE(b_type) => {
            if (registers[b_type.rs1() as usize] as i32)
                >= (registers[b_type.rs2() as usize] as i32)
            {
                next_pc = pc.overflowing_add(b_type.imm()).0;
            }
        }
        Instruction::BLTU(b_type) => {
            if registers[b_type.rs1() as usize] < registers[b_type.rs2() as usize] {
                next_pc = pc.overflowing_add(b_type.imm()).0;
            }
        }
        Instruction::BGEU(b_type) => {
            if registers[b_type.rs1() as usize] >= registers[b_type.rs2() as usize] {
                next_pc = pc.overflowing_add(b_type.imm()).0;
            }
        }
        // LOAD
        Instruction::LB(i_type) => {
            let address = registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0;
            rd = Some(i_type.rd());
            rd_value = sign_extend(load_word(memory, address) & 0xFF, 8);
        }
        Instruction::LH(i_type) => {
            let address = registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0;
            rd = Some(i_type.rd());
            rd_value = sign_extend(load_word(memory, address) & 0xFFFF, 16);
        }
        Instruction::LW(i_type) => {
            let address = registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0;
            rd = Some(i_type.rd());
            rd_value = load_word(memory, address);
        }
        Instruction::LBU(i_type) => {
            let address = registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0;
            rd = Some(i_type.rd());
            rd_value = load_word(memory, address) & 0xFF;
        }
        Instruction::LHU(i_type) => {
            let address = registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0;
            rd = Some(i_type.rd());
            rd_value = load_word(memory, address) & 0xFFFF;
        }
        // STORE
        Instruction::SB(s_type) => {
            let address = registers[s_type.rs1() as usize]
                .overflowing_add(s_type.imm())
                .0;
            store_byte(memory, address, registers[s_type.rs2() as usize] as u8)
        }
        Instruction::SH(s_type) => {
            let address = registers[s_type.rs1() as usize]
                .overflowing_add(s_type.imm())
                .0;
            store_half_word(memory, address, registers[s_type.rs2() as usize] as u16)
        }
        Instruction::SW(s_type) => {
            let address = registers[s_type.rs1() as usize]
                .overflowing_add(s_type.imm())
                .0;
            store_word(memory, address, registers[s_type.rs2() as usize])
        }
        // OP-IMM
        Instruction::ADDI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = registers[i_type.rs1() as usize]
                .overflowing_add(i_type.imm())
                .0;
        }
        Instruction::SLTI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = ((registers[i_type.rs1() as usize] as i32) < (i_type.imm() as i32)) as u32;
        }
        Instruction::SLTIU(i_type) => {
            rd = Some(i_type.rd());
            rd_value = (registers[i_type.rs1() as usize] < i_type.imm()) as u32;
        }
        Instruction::XORI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = registers[i_type.rs1() as usize] ^ i_type.imm();
        }
        Instruction::ORI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = registers[i_type.rs1() as usize] | i_type.imm();
        }
        Instruction::ANDI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = i_type.rs1() & i_type.imm();
        }
        Instruction::SLLI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = registers[i_type.rs1() as usize] << (i_type.imm() & 0b1111);
        }
        Instruction::SRLI(i_type) => {
            rd = Some(i_type.rd());
            rd_value = registers[i_type.rs1() as usize] >> (i_type.imm() & 0b1111);
        }
        Instruction::SRAI(i_type) => {
            rd = Some(i_type.rd());
            // rust uses arithmetic right shift on signed integer types
            rd_value =
                ((registers[i_type.rs1() as usize] as i32) >> (i_type.imm() & 0b1111)) as u32;
        }
        // OP
        Instruction::ADD(r_type) => {
            rd = Some(r_type.rd());
            rd_value = registers[r_type.rs1() as usize]
                .overflowing_add(registers[r_type.rs2() as usize])
                .0;
        }
        Instruction::SUB(r_type) => {
            rd = Some(r_type.rd());
            rd_value = registers[r_type.rs1() as usize]
                .overflowing_sub(registers[r_type.rs2() as usize])
                .0;
        }
        Instruction::SLL(r_type) => {
            rd = Some(r_type.rd());
            rd_value =
                registers[r_type.rs1() as usize] << (registers[r_type.rs2() as usize] & 0b11111);
        }
        Instruction::SLT(r_type) => {
            rd = Some(r_type.rd());
            rd_value = ((registers[r_type.rs1() as usize] as i32)
                < (registers[r_type.rs2() as usize] as i32)) as u32;
        }
        Instruction::SLTU(r_type) => {
            rd = Some(r_type.rd());
            rd_value = (registers[r_type.rs1() as usize] < registers[r_type.rs2() as usize]) as u32;
        }
        Instruction::XOR(r_type) => {
            rd = Some(r_type.rd());
            rd_value = registers[r_type.rs1() as usize] ^ registers[r_type.rs2() as usize]
        }
        Instruction::SRL(r_type) => {
            rd = Some(r_type.rd());
            rd_value =
                registers[r_type.rs1() as usize] >> (registers[r_type.rs2() as usize] & 0b11111);
        }
        Instruction::SRA(r_type) => {
            rd = Some(r_type.rd());
            // rust uses arithmetic right shift on signed integer types
            rd_value = ((registers[r_type.rs1() as usize] as i32)
                >> (registers[r_type.rs2() as usize] & 0b11111)) as u32;
        }
        Instruction::OR(r_type) => {
            rd = Some(r_type.rd());
            rd_value = registers[r_type.rs1() as usize] | registers[r_type.rs2() as usize]
        }
        Instruction::AND(r_type) => {
            rd = Some(r_type.rd());
            rd_value = registers[r_type.rs1() as usize] & registers[r_type.rs2() as usize]
        }
        // SYSTEM
        Instruction::ECALL | Instruction::EBREAK => {}
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
    // for entry in glob::glob("riscv-tests/isa/rv32ui*").unwrap() {
    //     let path = entry.unwrap();
    //     if path.is_dir() || path.extension().is_some() {
    //         continue;
    //     }
    //
    // }

    let path = std::path::Path::new("riscv-tests/isa/rv32ui-v-add");

    println!("ELF file: {:?}", path);

    let mut memory: Memory = [0; MEMORY_SIZE];
    let mut registers: Registers = [0; 33];
    load_elf(&mut memory, path);
    registers[PC] = MEMORY_START as u32;

    println!(
        "{:4} {:8} {:8} {:0}",
        "STEP", "ADDRESS", "CODE", "INSTRUCTION"
    );
    for i in 0..1_0000 {
        print!("{:4} {:8x} ", i, registers[PC]);
        step(&mut registers, &mut memory);
        if i == 32 {
            dump_registers(&registers);
        }
    }
}
