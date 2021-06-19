// pub enum Format {
//     RType,
//     IType,
//     SType,
//     BType,
//     UType,
//     JType,
// }

// struct Instruction(u32, Format);

use crate::formats::*;

#[derive(Debug)]
pub enum Instruction {
    // LUI 0110111
    LUI(UType),

    // AUPIC 0010111
    AUIPC(UType),

    // JAL 1101111
    JAL(JType),

    // JALR 1100111
    JALR(IType),

    // BRANCH 1100011
    BEQ(BType),
    BNE(BType),
    BLT(BType),
    BGE(BType),
    BLTU(BType),
    BGEU(BType),

    // LOAD 0000011
    LB(IType),
    LH(IType),
    LW(IType),
    LBU(IType),
    LHU(IType),

    // STORE 0100011
    SB(SType),
    SH(SType),
    SW(SType),

    // OP-IMM 0010011
    ADDI(IType),
    SLTI(IType),
    SLTIU(IType),
    XORI(IType),
    ORI(IType),
    ANDI(IType),
    SLLI(IType),
    SRLI(IType),
    SRAI(IType),

    // OP 0110011
    ADD(RType),
    SUB(RType),
    SLL(RType),
    SLT(RType),
    SLTU(RType),
    XOR(RType),
    SRL(RType),
    SRA(RType),
    OR(RType),
    AND(RType),

    // FENCE 0001111
    FENCE,

    // SYSTEM 1110011
    ECALL,
    EBREAK,
}
