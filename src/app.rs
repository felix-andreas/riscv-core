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

#[derive(Debug, Clone)]
enum RunningState {
    Idle,
    Running,
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
    let running_state = RwSignal::new(RunningState::Idle);
    let message = move || match state.get() {
        State::Errored(error) => match error {
            Error::MemoryError { address } => format!(
                "Memory Error: Address 0x{address:08x} \
                is outside of valid address range (0x{MEMORY_START:08x}-0x{:08x})",
                MEMORY_START + MEMORY_SIZE
            ),
            Error::DecodeError { code } => format!("Failed to decode instruction {code:016b}"),
        },
        _ => String::new(),
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
    let press_run_button = move |_| match running_state() {
        RunningState::Idle => running_state.set(RunningState::Running),
        RunningState::Running => running_state.set(RunningState::Idle),
    };

    reset();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div class="min-h-screen grid grid-rows-[auto_1fr_auto]">
            <div class="border border-b border-gray-200">
                <div class="p-4 mx-auto max-w-screen-xl border-x border-gray-200 flex items-center">
                    <div class="w-6"></div>
                    <div class="grow font-mono text-lg text-center">"RISC-V Exposed"</div>
                    <a href="https://github.com/felix-andreas/riscv-core" target="_blank">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 32 32"
                        >
                            <path
                                fill="currentColor"
                                fill-rule="evenodd"
                                d="M16 2a14 14 0 0 0-4.43 27.28c.7.13 1-.3 1-.67v-2.38c-3.89.84-4.71-1.88-4.71-1.88a3.71 3.71 0 0 0-1.62-2.05c-1.27-.86.1-.85.1-.85a2.94 2.94 0 0 1 2.14 1.45a3 3 0 0 0 4.08 1.16a2.93 2.93 0 0 1 .88-1.87c-3.1-.36-6.37-1.56-6.37-6.92a5.4 5.4 0 0 1 1.44-3.76a5 5 0 0 1 .14-3.7s1.17-.38 3.85 1.43a13.3 13.3 0 0 1 7 0c2.67-1.81 3.84-1.43 3.84-1.43a5 5 0 0 1 .14 3.7a5.4 5.4 0 0 1 1.44 3.76c0 5.38-3.27 6.56-6.39 6.91a3.33 3.33 0 0 1 .95 2.59v3.84c0 .46.25.81 1 .67A14 14 0 0 0 16 2"
                            ></path>
                        </svg>
                    </a>
                </div>
            </div>

            <div class="">
                <div class="h-full mx-auto max-w-screen-xl border-x border-gray-200">
                    <div class="p-4 flex gap-4">
                        <button
                            class=move || {
                                format!(
                                    "w-28 py-2 rounded-full font-semibold text-lg text-white disabled:opacity-50 flex justify-center items-center gap-2 {}",
                                    match running_state() {
                                        RunningState::Idle => "bg-green-600 hover:bg-green-500",
                                        RunningState::Running => "bg-red-600 hover:bg-red-500",
                                    },
                                )
                            }

                            disabled=move || matches!(state(), State::Finished | State::Errored(_))
                            on:click=press_run_button
                        >
                            {move || match running_state() {
                                RunningState::Idle => {
                                    view! {
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="16"
                                            height="16"
                                            viewBox="0 0 32 32"
                                        >
                                            <path
                                                fill="currentColor"
                                                d="M7 28a1 1 0 0 1-1-1V5a1 1 0 0 1 1.482-.876l20 11a1 1 0 0 1 0 1.752l-20 11A1 1 0 0 1 7 28"
                                            ></path>
                                        </svg>
                                        Run
                                    }
                                }
                                RunningState::Running => {
                                    view! {
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="16"
                                            height="16"
                                            viewBox="0 0 32 32"
                                        >
                                            <path
                                                fill="currentColor"
                                                d="M24 6H8a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2"
                                            ></path>
                                        </svg>
                                        Stop
                                    }
                                }
                            }}

                        </button>
                        <button
                            class="px-5 py-2 border-2 border-gray-700 rounded-full font-semibold text-lg disabled:opacity-50 flex items-center gap-3"
                            on:click=step
                            disabled=move || matches!(state(), State::Finished | State::Errored(_))
                        >

                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 32 32"
                            >
                                <path
                                    fill="currentColor"
                                    d="m18 6l-1.43 1.393L24.15 15H4v2h20.15l-7.58 7.573L18 26l10-10z"
                                ></path>
                            </svg>
                            Step
                        </button>
                        <button
                            class="px-5 py-2 bg-black rounded-full font-semibold text-lg text-white disabled:opacity-50 flex items-center gap-3"
                            on:click=move |_| reset()
                            disabled=move || matches!(state(), State::Fresh)
                        >

                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 32 32"
                            >
                                <path
                                    fill="currentColor"
                                    d="M18 28A12 12 0 1 0 6 16v6.2l-3.6-3.6L1 20l6 6l6-6l-1.4-1.4L8 22.2V16a10 10 0 1 1 10 10Z"
                                ></path>
                            </svg>
                            Reset
                        </button>
                    </div>

                    <div class="p-4 grid grid-cols-[auto_auto_auto] gap-4 items-start">
                        <Program memory=memory pc=pc/>
                        <Registers registers=registers/>
                        <Memory memory=memory/>
                    </div>

                    <div class="">{message}</div>
                </div>
            </div>

            <div class="border border-t border-gray-200">
                <div class="h-16 mx-auto max-w-screen-xl border-x border-gray-200 grid place-items-center">
                    <div class="opacity-35 text-xs">"RISC-V Exposed © 2024 Felix Andreas."</div>
                </div>
            </div>
        </div>
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
enum InstType {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum View {
    Hex,
    Binary,
    Decoded,
}

#[component]
pub fn Program(memory: RwSignal<Memory>, pc: Signal<u32>) -> impl IntoView {
    let start = move || pc() / 16 * 16;
    let program: Signal<Vec<u32>> = Signal::derive(move || {
        let memory = memory();
        (0..16)
            .filter_map(|i| {
                let address = start() + 4 * i;
                crate::utils::load_word(&memory, address).ok()
            })
            .collect()
    });
    let view_state = RwSignal::new(View::Binary);
    let view_instruction = move |i_type, code: u32| {
        view! {
            <div class="grid items-center gap-2 text-center divide-x divide-gray-700">
                <div
                    class="grid bg-gray-700 gap-px"
                    style="grid-template-columns: repeat(32, 1fr); height: 49px; width: 543px;"
                >
                    <Show when=move || { matches!(view_state(), View::Binary) }>
                        <For
                            each=move || {
                                (0..32)
                                    .rev()
                                    .map(|n| (code >> n) & 1)
                                    .enumerate()
                                    .collect::<Vec<_>>()
                            }

                            key=|(i, _)| *i
                            let:child
                        >
                            <div class="w-4 h-6 bg-white text-sm font-mono grid place-items-center">
                                {child.1}
                            </div>
                        </For>
                    </Show>

                    <Show when=move || { matches!(view_state(), View::Hex) }>
                        <div class="bg-white grid col-span-full place-items-center h-full font-mono">
                            {format!("{code:08x}")}
                        </div>
                    </Show>
                    {match i_type {
                        Some(InstType::RType(r_type)) => {
                            view! {
                                <>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "f7"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs2"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs1"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-3">
                                        "f3"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rd"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "opcode"
                                    </div>
                                </>
                            }
                        }
                        Some(InstType::IType(i_type)) => {
                            view! {
                                <>
                                    <div class="h-6 bg-white grid place-items-center col-span-12">
                                        "imm"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs1"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-3">
                                        "f3"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rd"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "opcode"
                                    </div>
                                </>
                            }
                        }
                        Some(InstType::SType(s_type)) => {
                            view! {
                                <>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "imm"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs2"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs1"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-3">
                                        "f3"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "imm"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "opcode"
                                    </div>
                                </>
                            }
                        }
                        Some(InstType::BType(b_type)) => {
                            view! {
                                <>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "imm"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs2"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rs1"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-3">
                                        "f3"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "imm"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "opcode"
                                    </div>
                                </>
                            }
                        }
                        Some(InstType::UType(u_type)) => {
                            view! {
                                <>
                                    <div
                                        class="h-6 bg-white grid place-items-center"
                                        style="grid-column: span 20;"
                                    >
                                        "imma"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rd"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "opcode"
                                    </div>
                                </>
                            }
                        }
                        Some(InstType::JType(j_type)) => {
                            view! {
                                <>
                                    <div
                                        class="h-6 bg-white grid place-items-center"
                                        style="grid-column: span 20"
                                    >
                                        "imm"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-5">
                                        "rd"
                                    </div>
                                    <div class="h-6 bg-white grid place-items-center col-span-7">
                                        "opcode"
                                    </div>
                                </>
                            }
                        }
                        None => {
                            view! {
                                <>
                                    <div class="h-6 bg-white grid place-items-center col-span-full">
                                        "unknown"
                                    </div>
                                </>
                            }
                        }
                    }}

                </div>

            </div>
        }
    };

    view! {
        <div class="grid gap-2">
            <div class="opacity-50 text-lg text-center">"Program"</div>
            <div class="grid grid-cols-3 border-2 border-gray-900 overflow-hidden">
                {[View::Binary, View::Hex, View::Decoded]
                    .map(|x| {
                        view! {
                            <button
                                on:click=move |_| view_state.set(x)
                                class=move || {
                                    format!(
                                        "p1 {}",
                                        if x == view_state() {
                                            "text-white font-medium bg-gray-900"
                                        } else {
                                            ""
                                        },
                                    )
                                }
                            >

                                {match x {
                                    View::Binary => "Binary",
                                    View::Hex => "Hex",
                                    View::Decoded => "Decoded",
                                }}

                            </button>
                        }
                    })}

            </div>
            <div class="">
                <div class="grid grid-cols-[2fr_2fr_auto] gap-x-px font-medium border-t-2 border-x-2 border-gray-700 bg-gray-700">
                    <div class="bg-gray-100 px-1 text-center">addr</div>
                    <div class="bg-gray-100 px-1 text-center">instr</div>
                    <div class="bg-gray-100 px-1 text-center" style="width: 543px;">
                        {move || match view_state() {
                            View::Binary => "Binary",
                            View::Hex => "Hex",
                            View::Decoded => "Decoded",
                        }}

                    </div>
                </div>
                <div class="border-2 border-gray-700 bg-gray-700 grid gap-px">
                    <For
                        each=move || program().into_iter().enumerate()
                        key=|(index, _)| *index
                        let:child
                    >
                        <div
                            class="grid grid-cols-[2fr_2fr_auto] gap-px bg-gray-700"
                            class=("bg-gray-200", move || 4 * child.0 as u32 + start() == pc())
                        >

                            {
                                let (name, i_type) = code_to_name(child.1);
                                view! {
                                    <p class="bg-white grid place-items-center">
                                        {format!("{:02x}", 4 * child.0)}
                                    </p>
                                    <p class="bg-white grid place-items-center">{name}</p>
                                    <p>{view_instruction(i_type, child.1)}</p>
                                }
                            }

                        </div>
                    </For>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Registers(registers: RwSignal<Registers>) -> impl IntoView {
    view! {
        <div class="grid gap-2 place-items-center">
            <div class="text-center text-lg opacity-50">"Registers"</div>
            <div class="grid gap-y-px border-2 border-gray-500 bg-gray-700 font-mono">
                <div class="grid grid-cols-[3rem_auto] bg-gray-100">
                    <p class="text-right px-2 border-r-2 border-gray-700 ">"reg"</p>
                    <div class="text-center">"value"</div>
                </div>
                <div class="grid grid-cols-[3rem_auto] bg-white">
                    <p class="text-right px-2 border-r-2 border-gray-700 font-semibold">"pc"</p>
                    {view_word(registers()[PC])}
                </div>
                <For
                    each=move || registers().into_iter().take(PC).enumerate()
                    key=|(index, _)| *index
                    let:child
                >
                    <div class="grid grid-cols-[3rem_auto] bg-white">
                        <p class="text-right px-2 border-r-2 border-gray-700 font-semibold">
                            {format!("x{}", child.0)}
                        </p>
                        {view_word(child.1)}
                    </div>
                </For>
            </div>
        </div>
    }
}

pub fn view_word(word: u32) -> impl IntoView {
    let a = word & 0xff;
    let b = (word >> 8) & 0xff;
    let c = (word >> 16) & 0xff;
    let d = (word >> 24) & 0xff;
    view! {
        <div class="grid gap-x-px grid-cols-4 place-items-center bg-gray-700">
            <div class="w-8 bg-white text-center">{format!("{a:02x}")}</div>
            <div class="w-8 bg-white text-center">{format!("{b:02x}")}</div>
            <div class="w-8 bg-white text-center">{format!("{c:02x}")}</div>
            <div class="w-8 bg-white text-center">{format!("{d:02x}")}</div>
        </div>
    }
}

#[component]
pub fn Memory(memory: RwSignal<Memory>) -> impl IntoView {
    view! {
        <div class="grid gap-2 place-items-center">
            <div class="font-bold">"RAM"</div>
            <div class="font-mono grid grid-cols-[3fr_2fr_2fr_2fr_2fr] gap-px bg-gray-700 border-2 border-gray-700">
                <span class="bg-gray-100 text-center border-r border-gray-700">"addr"</span>
                <span class="bg-gray-100 text-center col-span-4">"value"</span>
                <For
                    each=move || {
                        memory
                            .with(|memory| {
                                memory
                                    .chunks(4)
                                    .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
                                    .take(33)
                                    .enumerate()
                                    .collect::<Vec<_>>()
                            })
                    }

                    key=|(index, _)| *index
                    let:child
                >
                    <span class="w-12 bg-white text-center font-semibold border-r border-gray-700">
                        {format!("{:02x}", 4 * child.0)}
                    </span>
                    <span class="w-8 bg-white text-center">{format!("{:02x}", child.1[0])}</span>
                    <span class="w-8 bg-white text-center">{format!("{:02x}", child.1[1])}</span>
                    <span class="w-8 bg-white text-center">{format!("{:02x}", child.1[2])}</span>
                    <span class="w-8 bg-white text-center">{format!("{:02x}", child.1[3])}</span>
                </For>
            </div>
        </div>
    }
}

// region: Utils

fn code_to_name(code: u32) -> (&'static str, Option<InstType>) {
    match crate::decode(code) {
        None => ("UNK", None),
        Some(instruction) => match instruction {
            Instruction::LUI(lui) => ("LUI", Some(InstType::UType(lui))),
            Instruction::AUIPC(auipc) => ("AUIPC", Some(InstType::UType(auipc))),
            Instruction::JAL(jal) => ("JAL", Some(InstType::JType(jal))),
            Instruction::JALR(jalr) => ("JALR", Some(InstType::IType(jalr))),
            Instruction::BEQ(beq) => ("BEQ", Some(InstType::BType(beq))),
            Instruction::BNE(bne) => ("BNE", Some(InstType::BType(bne))),
            Instruction::BLT(blt) => ("BLT", Some(InstType::BType(blt))),
            Instruction::BGE(bge) => ("BGE", Some(InstType::BType(bge))),
            Instruction::BLTU(bltu) => ("BLTU", Some(InstType::BType(bltu))),
            Instruction::BGEU(bgeu) => ("BGEU", Some(InstType::BType(bgeu))),
            Instruction::LB(lb) => ("LB", Some(InstType::IType(lb))),
            Instruction::LH(lh) => ("LH", Some(InstType::IType(lh))),
            Instruction::LW(lw) => ("LW", Some(InstType::IType(lw))),
            Instruction::LBU(lbu) => ("LBU", Some(InstType::IType(lbu))),
            Instruction::LHU(lhu) => ("LHU", Some(InstType::IType(lhu))),
            Instruction::SB(sb) => ("SB", Some(InstType::SType(sb))),
            Instruction::SH(sh) => ("SH", Some(InstType::SType(sh))),
            Instruction::SW(sw) => ("SW", Some(InstType::SType(sw))),
            Instruction::ADDI(addi) => ("ADDI", Some(InstType::IType(addi))),
            Instruction::SLTI(slti) => ("SLTI", Some(InstType::IType(slti))),
            Instruction::SLTIU(sltiu) => ("SLTIU", Some(InstType::IType(sltiu))),
            Instruction::XORI(xori) => ("XORI", Some(InstType::IType(xori))),
            Instruction::ORI(ori) => ("ORI", Some(InstType::IType(ori))),
            Instruction::ANDI(andi) => ("ANDI", Some(InstType::IType(andi))),
            Instruction::SLLI(slli) => ("SLLI", Some(InstType::IType(slli))),
            Instruction::SRLI(srli) => ("SRLI", Some(InstType::IType(srli))),
            Instruction::SRAI(srai) => ("SRAI", Some(InstType::IType(srai))),
            Instruction::ADD(add) => ("ADD", Some(InstType::RType(add))),
            Instruction::SUB(sub) => ("SUB", Some(InstType::RType(sub))),
            Instruction::SLL(sll) => ("SLL", Some(InstType::RType(sll))),
            Instruction::SLT(slt) => ("SLT", Some(InstType::RType(slt))),
            Instruction::SLTU(sltu) => ("SLTU", Some(InstType::RType(sltu))),
            Instruction::XOR(xor) => ("XOR", Some(InstType::RType(xor))),
            Instruction::SRL(srl) => ("SRL", Some(InstType::RType(srl))),
            Instruction::SRA(sra) => ("SRA", Some(InstType::RType(sra))),
            Instruction::OR(or) => ("OR", Some(InstType::RType(or))),
            Instruction::AND(and) => ("AND", Some(InstType::RType(and))),
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
        },
    }
}

// endregion
