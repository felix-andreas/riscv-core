#![allow(non_snake_case)]

use leptos::*;
use leptos_meta::*;

use crate::{
    formats::{BType, IType, JType, RType, SType, UType},
    Error, Instruction, Memory, Registers, MEMORY_SIZE, MEMORY_START, PC,
};

#[derive(Debug, Clone)]
enum State {
    Fresh,
    Started,
    Finished,
    Errored(Error),
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let code: Vec<u8> = [
        0xfd010113, 0x02812623, 0x03010413, 0x00a00793, 0xfef42023, 0xfe042623, 0x00100793,
        0xfef42423, 0xfe042783, 0x00079663, 0xfec42783, 0x04c0006f,
    ]
    .map(u32::to_le_bytes)
    .into_iter()
    .flatten()
    .collect();

    let state = RwSignal::new(State::Fresh);
    let message = move || match state.get() {
        State::Errored(error) => match error {
            Error::MemoryError { address } => format!(
                "Memory Error: Address 0x{address:08x} \
                is outside of valid address range (0x{MEMORY_START:08x}-0x{:08x})",
                MEMORY_START + MEMORY_SIZE
            ),
            Error::DecodeError { code } => format!("Failed to decode instruction {code:016b}"),
        },
        _ => format!(""),
    };

    let registers: RwSignal<Registers> = RwSignal::new([0; 33]);
    let pc = Signal::derive(move || registers()[PC]);
    let memory: RwSignal<Memory> = RwSignal::new([0; MEMORY_SIZE]);

    let reset = move || {
        state.set(State::Fresh);
        registers.update(|registers| {
            registers.copy_from_slice(&[0; 33]);
            registers[PC] = MEMORY_START as u32;
        });
        memory.update(|memory| {
            memory.copy_from_slice(&[0; MEMORY_SIZE]);
            memory[..code.len()].copy_from_slice(&code);
        });
    };

    let step = move |_| {
        logging::log!("pc {}", registers()[PC]);

        registers.update(|registers| {
            let result = crate::step(registers, &mut memory());
            logging::log!("result {:?}", result);
            state.set(match result {
                Ok(false) => State::Started,
                Ok(true) => State::Finished,
                Err(error) => State::Errored(error),
            });
        });
        memory.update(|_| {});
        logging::log!("after {}", registers()[PC]);
    };

    reset();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div class="min-h-screen grid grid-rows-[auto_1fr_auto]">
            <div class="border border-b border-gray-200">
                <div class="h-16 mx-auto max-w-screen-lg border-x border-gray-200"></div>
            </div>

            <div class="">
                <div class="h-full mx-auto max-w-screen-lg border-x border-gray-200">
                    <button
                        class="m-4 px-5 py-2 bg-black rounded-full font-semibold text-lg text-white disabled:opacity-50"
                        on:click=step
                        disabled=move || matches!(state(), State::Finished | State::Errored(_))
                    >

                        Step
                    </button>
                    <button
                        class="ml-2 m-4 px-5 py-2 bg-black rounded-full font-semibold text-lg text-white disabled:opacity-50"
                        on:click=move |_| reset()
                        disabled=move || matches!(state(), State::Fresh)
                    >

                        Reset
                    </button>

                    <div class="p-4 grid sm:grid-cols-[2fr_1fr_1fr] items-start">
                        <Program memory=memory pc=pc/>
                        <Registers registers=registers/>
                        <Memory memory=memory/>
                    </div>

                    <div class="">{message}</div>
                </div>
            </div>

            <div class="border border-t border-gray-200">
                <div class="h-16 mx-auto max-w-screen-lg border-x border-gray-200"></div>
            </div>
        </div>
    }
}

#[component]
pub fn Registers(registers: RwSignal<Registers>) -> impl IntoView {
    view! {
        <div class="grid place-items-center font-mono">
            <div class="font-bold">"Registers"</div>
            <div class="grid grid-cols-[auto_1fr] gap-x-1">
                <For
                    each=move || registers().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:child
                >
                    <p>{format!("x{}", child.0)}</p>
                    <p>{format!("0x{:08x}", child.1)}</p>
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn Memory(memory: RwSignal<Memory>) -> impl IntoView {
    view! {
        <div class="grid place-items-center font-mono">
            <div class="font-bold">"RAM"</div>
            <div class="grid grid-cols-[auto_1fr] gap-x-1">
                <For
                    each=move || memory()[0..32].to_vec().into_iter().take(33).enumerate()
                    key=|(index, _)| *index
                    let:child
                >
                    <p>{format!("x{:02x}", child.0)}</p>
                    <p>{format!("0x{:08x}", child.1)}</p>
                </For>
            </div>
        </div>
    }
}

enum Type {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
}

#[component]
pub fn Program(memory: RwSignal<Memory>, pc: Signal<u32>) -> impl IntoView {
    let start = move || pc() / 32 * 32;
    let program: Signal<Vec<u32>> = Signal::derive(move || {
        let memory = memory();
        (0..32)
            .filter_map(|i| {
                let address = start() + 4 * i;
                crate::utils::load_word(&memory, address as u32).ok()
            })
            .collect()
    });
    let view_instruction = |i_type| match i_type {
        Some(Type::RType(r_type)) => {
            view! {
                <div>
                    <div>{r_type.rd()}</div>
                    <div>{r_type.rs1()}</div>
                    <div>{r_type.rs2()}</div>
                </div>
            }
        }
        Some(Type::IType(i_type)) => {
            view! {
                <div class="flex items-center border border-gray-700 gap-2">
                    <div>"IType"</div>
                    <div>"IMM " {i_type.imm()}</div>
                    <div class="border">{i_type.rd()}</div>
                    <div>{i_type.rs1()}</div>
                </div>
            }
        }
        None => view! { <div>-</div> },
        _ => view! { <div>other</div> },
    };

    view! {
        <div class="grid font-mono gap-2">
            <div class="font-bold text-center">"Program"</div>
            <div class="grid grid-cols-[1fr_2fr_1fr_5fr] gap-x-2 font-medium">
                <div>Addr</div>
                <div>Raw</div>
                <div>Instr</div>
                <div>Decoded</div>
            </div>
            <div>
                <For
                    each=move || program().into_iter().enumerate()
                    key=|(index, _)| *index
                    let:child
                >
                    <div
                        class="grid grid-cols-[1fr_2fr_1fr_5fr] gap-x-2"
                        class=("bg-gray-200", move || 4 * child.0 as u32 + start() == pc())
                    >
                        {match crate::decode(child.1) {
                            None => {
                                view! {
                                    <p>{format!("{:02x}", child.0)}</p>
                                    <p>{format!("0x{:08x}", child.1)}</p>
                                    <p>unknown</p>
                                    <p>unknown</p>
                                }
                            }
                            Some(instruction) => {
                                let (name, i_type) = match instruction {
                                    Instruction::LUI(lui) => ("LUI", Some(Type::UType(lui))),
                                    Instruction::AUIPC(auipc) => ("AUIPC", Some(Type::UType(auipc))),
                                    Instruction::JAL(jal) => ("JAL", Some(Type::JType(jal))),
                                    Instruction::JALR(jalr) => ("JALR", Some(Type::IType(jalr))),
                                    Instruction::BEQ(beq) => ("BEQ", Some(Type::BType(beq))),
                                    Instruction::BNE(bne) => ("BNE", Some(Type::BType(bne))),
                                    Instruction::BLT(blt) => ("BLT", Some(Type::BType(blt))),
                                    Instruction::BGE(bge) => ("BGE", Some(Type::BType(bge))),
                                    Instruction::BLTU(bltu) => ("BLTU", Some(Type::BType(bltu))),
                                    Instruction::BGEU(bgeu) => ("BGEU", Some(Type::BType(bgeu))),
                                    Instruction::LB(lb) => ("LB", Some(Type::IType(lb))),
                                    Instruction::LH(lh) => ("LH", Some(Type::IType(lh))),
                                    Instruction::LW(lw) => ("LW", Some(Type::IType(lw))),
                                    Instruction::LBU(lbu) => ("LBU", Some(Type::IType(lbu))),
                                    Instruction::LHU(lhu) => ("LHU", Some(Type::IType(lhu))),
                                    Instruction::SB(sb) => ("SB", Some(Type::SType(sb))),
                                    Instruction::SH(sh) => ("SH", Some(Type::SType(sh))),
                                    Instruction::SW(sw) => ("SW", Some(Type::SType(sw))),
                                    Instruction::ADDI(addi) => ("ADDI", Some(Type::IType(addi))),
                                    Instruction::SLTI(slti) => ("SLTI", Some(Type::IType(slti))),
                                    Instruction::SLTIU(sltiu) => ("SLTIU", Some(Type::IType(sltiu))),
                                    Instruction::XORI(xori) => ("XORI", Some(Type::IType(xori))),
                                    Instruction::ORI(ori) => ("ORI", Some(Type::IType(ori))),
                                    Instruction::ANDI(andi) => ("ANDI", Some(Type::IType(andi))),
                                    Instruction::SLLI(slli) => ("SLLI", Some(Type::IType(slli))),
                                    Instruction::SRLI(srli) => ("SRLI", Some(Type::IType(srli))),
                                    Instruction::SRAI(srai) => ("SRAI", Some(Type::IType(srai))),
                                    Instruction::ADD(add) => ("ADD", Some(Type::RType(add))),
                                    Instruction::SUB(sub) => ("SUB", Some(Type::RType(sub))),
                                    Instruction::SLL(sll) => ("SLL", Some(Type::RType(sll))),
                                    Instruction::SLT(slt) => ("SLT", Some(Type::RType(slt))),
                                    Instruction::SLTU(sltu) => ("SLTU", Some(Type::RType(sltu))),
                                    Instruction::XOR(xor) => ("XOR", Some(Type::RType(xor))),
                                    Instruction::SRL(srl) => ("SRL", Some(Type::RType(srl))),
                                    Instruction::SRA(sra) => ("SRA", Some(Type::RType(sra))),
                                    Instruction::OR(or) => ("OR", Some(Type::RType(or))),
                                    Instruction::AND(and) => ("AND", Some(Type::RType(and))),
                                    Instruction::FENCE => ("FENCE", None),
                                    Instruction::ECALL => ("ECALL", None),
                                    Instruction::EBREAK => ("EBREAK", None),
                                    Instruction::URET => ("URET", None),
                                    Instruction::SRET => ("SRET", None),
                                    Instruction::MRET => ("MRET", None),
                                    Instruction::WFI => ("WFI", None),
                                    Instruction::CSRRW => ("CSRRW", None),
                                    Instruction::CSRRS => ("CSRRS", None),
                                    Instruction::CSRRC => ("CSRRC", None),
                                    Instruction::CSRRWI => ("CSRRWI", None),
                                    Instruction::CSRRSI => ("CSRRSI", None),
                                    Instruction::CSRRCI => ("CSRRCI", None),
                                };
                                view! {
                                    <p>{format!("{:02x}", child.0)}</p>
                                    <p>{format!("0x{:08x}", child.1)}</p>
                                    <p>{name}</p>
                                    <p>{view_instruction(i_type)}</p>
                                }
                            }
                        }}

                    </div>
                </For>
            </div>
        </div>
    }
}
