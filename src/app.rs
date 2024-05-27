#![allow(non_snake_case)]

use std::time::Duration;

use leptos::{leptos_dom::helpers::IntervalHandle, *};
use leptos_meta::*;

use crate::{
    formats::{BType, IType, JType, RType, SType, UType},
    utils::REGISTER_NAMES,
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
    Running(IntervalHandle),
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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

    let programs: &[(&'static str, &'static [u32])] = &[
        (
            "Fibonacci",
            &[
                0xff010113, 0x02812623, 0x03010413, 0x00a00793, 0xfef42023, 0xfe042623, 0x00100793,
                0xfef42423, 0xfe042783, 0x00079663, 0xfec42783, 0x04c0006f,
            ],
        ),
        (
            "fib_opt",
            &[
                0x00900793, 0x00100513, 0x00000713, 0x00050693, 0x00e50533, 0xfff78793, 0x00068713,
                0xfe0798e3, 0x00008067,
            ],
        ),
        (
            "Bubble sort",
            &[
                0xff010113, 0x00500793, 0x00f12023, 0x00100793, 0x00f12223, 0x00200793, 0x00f12423,
                0x00400793, 0x00f12623, 0x00010513, 0x00c10613, 0x00000893, 0x00100813, 0x0340006f,
                0x00478793, 0x02c78063, 0x0007a703, 0x0047a683, 0xfee6d8e3, 0x00d7a023, 0x00e7a223,
                0x00080593, 0xfe1ff06f, 0x00058c63, 0xffc60613, 0x00a60863, 0x00050793, 0x00088593,
                0xfd1ff06f, 0x01010113, 0x00008067,
            ],
        ),
    ];

    let selected_program = RwSignal::new(0);
    let frequencies = [1, 2, 4, 8, 16];
    let frequency = RwSignal::new(frequencies[1]);

    let stop = move || match running_state.get_untracked() {
        RunningState::Idle => {}
        RunningState::Running(handle) => {
            handle.clear();
            running_state.set(RunningState::Idle)
        }
    };

    let reset = move || {
        stop();
        state.set(State::Fresh);
        registers.update(|registers| {
            registers.copy_from_slice(&[0; 33]);
            registers[PC] = MEMORY_START as u32;
            registers[2] = MEMORY_START as u32 + 0xa0;
        });
        memory.update(|memory| {
            memory.copy_from_slice(&[0; MEMORY_SIZE]);
            let program = programs[selected_program.get_untracked()]
                .1
                .iter()
                .copied()
                .map(u32::to_le_bytes)
                .flatten()
                .collect::<Vec<u8>>();
            for (m, p) in memory[..program.len()].iter_mut().zip(program) {
                *m = p;
            }
        });
    };

    let load = move |index| {
        selected_program.set(index);
        reset();
    };

    let step = move || {
        leptos::batch(|| {
            update!(|registers, memory| {
                let result = crate::step(registers, memory);
                state.set(match result {
                    Ok(false) => State::Started,
                    Ok(true) => {
                        stop();
                        State::Finished
                    }
                    Err(error) => {
                        stop();
                        State::Errored(error)
                    }
                });
            });
        });
    };

    let press_run_button = move |_| match running_state() {
        RunningState::Idle => {
            running_state.set(RunningState::Running(
                leptos::set_interval_with_handle(
                    move || step(),
                    Duration::from_secs_f64(1.0 / frequency() as f64),
                )
                .unwrap(),
            ));
        }
        RunningState::Running(_) => stop(),
    };

    reset();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <div class="min-h-screen grid grid-rows-[auto_auto_1fr] bg-gray-50">
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
                <div class="h-full mx-auto max-w-screen-xl border-x border-gray-200 grid place-items-center">
                    <div class="p-8 grid gap-4">
                        <div class="grid gap-4">
                            <div class="grid gap-4 grid-cols-2 items-start">
                                <div class="p-4 bg-white ring-1 ring-gray-500/5 shadow-sm">
                                    <Program memory=memory pc=pc/>
                                </div>
                                <div class="p-4 bg-white ring-1 ring-gray-500/5 shadow-sm">
                                    <Registers registers=registers/>
                                </div>
                            </div>
                            <Memory memory=memory/>
                        </div>

                        <div class="flex gap-4 p-4 bg-white ring-1 ring-gray-500/5 shadow-sm">
                            <button
                                class=move || {
                                    format!(
                                        "w-28 py-1 font-medium text-lg text-white disabled:opacity-50 flex justify-center items-center gap-2 {}",
                                        match running_state() {
                                            RunningState::Idle => "bg-green-600 hover:bg-green-500",
                                            RunningState::Running(_) => "bg-red-600 hover:bg-red-500",
                                        },
                                    )
                                }

                                disabled=move || {
                                    matches!(state(), State::Finished | State::Errored(_))
                                }

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
                                    RunningState::Running(_) => {
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
                                class="px-5 py-2 border-2 border-gray-900 font-medium text-lg disabled:opacity-50 flex items-center gap-3"
                                on:click=move |_| step()
                                disabled=move || {
                                    matches!(state(), State::Finished | State::Errored(_))
                                }
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
                                class="px-5 py-2 bg-black font-medium text-lg text-white disabled:opacity-15 flex items-center gap-3"
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
                                "Reset"
                            </button>

                            <div class="ml-auto grid place-items-center opacity-15">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="20"
                                    height="20"
                                    viewBox="0 0 32 32"
                                    class="rounded-full transform -rotate-90"
                                >
                                    <circle
                                        style=move || {
                                            match running_state() {
                                                RunningState::Idle => "".into(),
                                                RunningState::Running(_) => {
                                                    format!(
                                                        "animation: clock-animation {}s linear infinite;",
                                                        1.0 / frequency() as f64,
                                                    )
                                                }
                                            }
                                        }

                                        class="animation"
                                        cx="16"
                                        cy="16"
                                        r="16"
                                    ></circle>
                                </svg>
                            </div>

                            <div class="relative">
                                <button
                                    popovertarget="programs-frequency"
                                    class="pl-7 pr-4 py-2 border-2 border-gray-900 font-medium text-lg disabled:opacity-50 flex items-center gap-3 hover:bg-gray-50"
                                    on:click=move |_| {}
                                >
                                    {move || format!("{} Hz", frequency())}
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 32 32"
                                    >
                                        <path
                                            fill="currentColor"
                                            d="M16 22L6 12l1.4-1.4l8.6 8.6l8.6-8.6L26 12z"
                                        ></path>
                                    </svg>
                                </button>
                                <div
                                    id="programs-frequency"
                                    popover
                                    class="p-0 top-0 right-0 bg-white border-2 border-gray-900 shadow-2xl"
                                >
                                    <div class="grid">
                                        <div class="text-center bg-black text-white px-24 py-6 font-semibold">
                                            "CPU frequency"
                                        </div>
                                        {frequencies
                                            .iter()
                                            .enumerate()
                                            .map(|(i, x)| {
                                                view! {
                                                    <button
                                                        class="p-4 hover:bg-gray-100"
                                                        on:click=move |_| frequency.set(frequencies[i])
                                                    >
                                                        {format!("{x} Hz")}
                                                    </button>
                                                }
                                            })
                                            .collect_view()}

                                    </div>
                                </div>
                            </div>

                            <div class="relative">
                                <button
                                    popovertarget="programs-dropdown"
                                    class="pl-7 pr-4 py-2 border-2 border-gray-900 font-medium text-lg disabled:opacity-50 flex items-center gap-3 hover:bg-gray-50"
                                    on:click=move |_| {}
                                >
                                    {move || programs[selected_program()].0}
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 32 32"
                                    >
                                        <path
                                            fill="currentColor"
                                            d="M16 22L6 12l1.4-1.4l8.6 8.6l8.6-8.6L26 12z"
                                        ></path>
                                    </svg>
                                </button>
                                <div
                                    id="programs-dropdown"
                                    popover
                                    class="p-0 top-0 right-0 bg-white border-2 border-gray-900 shadow-lg"
                                >
                                    <div class="grid">
                                        <div class="text-center bg-black text-white px-24 py-6 font-semibold">
                                            "Select a program"
                                        </div>
                                        {programs
                                            .iter()
                                            .enumerate()
                                            .map(|(i, x)| {
                                                view! {
                                                    <button
                                                        class="p-4 hover:bg-gray-100"
                                                        on:click=move |_| load(i)
                                                    >
                                                        {format!("{}", x.0)}
                                                    </button>
                                                }
                                            })
                                            .collect_view()}

                                    </div>
                                </div>
                            </div>
                        </div>

                        <Show when=move || !message().is_empty()>
                            <div class="p-8 border bg-red-50 border-red-200 text-red-900 flex items-center gap-2">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 32 32"
                                >
                                    <path
                                        fill="currentColor"
                                        d="M2 16A14 14 0 1 0 16 2A14 14 0 0 0 2 16m23.15 7.75L8.25 6.85a12 12 0 0 1 16.9 16.9M8.24 25.16a12 12 0 0 1-1.4-16.89l16.89 16.89a12 12 0 0 1-15.49 0"
                                    ></path>
                                </svg>
                                <div>{message}</div>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>

            <div class="grow border border-t border-gray-200">
                <div class="h-full mx-auto max-w-screen-xl border-x border-gray-200 grid justify-center">
                    <div class="py-8 opacity-35 text-xs">
                        "RISC-V Exposed © 2024 Felix Andreas."
                    </div>
                </div>
            </div>
        </div>
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy)]
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
    let n = 8;
    let start = move || pc() / (4 * n) * (4 * n);
    let program: Memo<Vec<u32>> = create_memo(move |_| {
        logging::log!("calc program!");
        (0..n)
            .filter_map(|i| {
                let address = start() + 4 * i;
                crate::utils::load_word(&memory(), address).ok()
            })
            .collect()
    });
    let view_state = RwSignal::new(View::Hex);
    let view_instruction = move |i_type, code: u32| {
        let opcode = code & 0b111_1111;
        let funct3 = code >> 12 & 0b111;
        let funct7 = code >> 25 & 0b111_1111;
        view! {
            <div
                class="grid bg-gray-200 gap-px"
                style="grid-template-columns: repeat(32, 1fr); height: 52px; width: 415px;"
            >
                <Show when=move || { matches!(view_state(), View::Binary) }>
                    <For
                        each=move || {
                            (0..32).rev().map(|n| (code >> n) & 1).enumerate().collect::<Vec<_>>()
                        }

                        key=|(i, _)| *i
                        let:child
                    >
                        <div class="w-3 h-6 bg-gray-50 text-sm font-mono grid place-items-center">
                            {child.1}
                        </div>
                    </For>
                </Show>

                <Show when=move || { matches!(view_state(), View::Hex) }>
                    <div class="bg-white grid col-span-full place-items-center h-full font-mono">
                        {format!("{code:08x}")}
                    </div>
                </Show>

                <Show when=move || { !matches!(view_state(), View::Hex) }>
                    <For
                        each=move || {
                            (match i_type {
                                Some(InstType::RType(r_type)) => {
                                    vec![
                                        ("f7", 7, format!("{funct7:07b}")),
                                        (
                                            "rs2",
                                            5,
                                            REGISTER_NAMES[r_type.rs2() as usize].to_string(),
                                        ),
                                        (
                                            "rs1",
                                            5,
                                            REGISTER_NAMES[r_type.rs1() as usize].to_string(),
                                        ),
                                        ("f3", 3, format!("{funct3:03b}")),
                                        ("rd", 5, REGISTER_NAMES[r_type.rd() as usize].to_string()),
                                        ("opcode", 7, format!("{opcode:07b}")),
                                    ]
                                }
                                Some(InstType::IType(i_type)) => {
                                    vec![
                                        ("imm", 12, format!("0x{:x}", i_type.imm())),
                                        (
                                            "rs1",
                                            5,
                                            REGISTER_NAMES[i_type.rs1() as usize].to_string(),
                                        ),
                                        ("f3", 3, format!("{funct3:03b}")),
                                        ("rd", 5, REGISTER_NAMES[i_type.rd() as usize].to_string()),
                                        ("opcode", 7, format!("{opcode:07b}")),
                                    ]
                                }
                                Some(InstType::SType(s_type)) => {
                                    vec![
                                        ("imm", 7, format!("0x{:x}", s_type.imm())),
                                        (
                                            "rs2",
                                            5,
                                            REGISTER_NAMES[s_type.rs2() as usize].to_string(),
                                        ),
                                        (
                                            "rs1",
                                            5,
                                            REGISTER_NAMES[s_type.rs1() as usize].to_string(),
                                        ),
                                        ("f3", 3, format!("{funct3:03b}")),
                                        ("imm", 5, format!("")),
                                        ("opcode", 7, format!("{opcode:07b}")),
                                    ]
                                }
                                Some(InstType::BType(b_type)) => {
                                    vec![
                                        ("imm", 7, format!("0x{:x}", b_type.imm())),
                                        (
                                            "rs2",
                                            5,
                                            REGISTER_NAMES[b_type.rs2() as usize].to_string(),
                                        ),
                                        (
                                            "rs1",
                                            5,
                                            REGISTER_NAMES[b_type.rs1() as usize].to_string(),
                                        ),
                                        ("f3", 3, format!("{funct3:03b}")),
                                        ("imm", 5, format!("")),
                                        ("opcode", 7, format!("{opcode:07b}")),
                                    ]
                                }
                                Some(InstType::UType(u_type)) => {
                                    vec![
                                        ("imm", 20, format!("0x{:x}", u_type.imm())),
                                        ("rd", 5, REGISTER_NAMES[u_type.rd() as usize].to_string()),
                                        ("opcode", 7, format!("{opcode:07b}")),
                                    ]
                                }
                                Some(InstType::JType(j_type)) => {
                                    vec![
                                        ("imm", 20, format!("0x{:x}", j_type.imm())),
                                        ("rd", 5, REGISTER_NAMES[j_type.rd() as usize].to_string()),
                                        ("opcode", 7, format!("{opcode:07b}")),
                                    ]
                                }
                                None => vec![("unknown", 32, format!("0x{code:08x}"))],
                            })
                                .into_iter()
                                .enumerate()
                                .collect::<Vec<_>>()
                        }

                        key=|(index, _)| *index
                        let:child
                    >
                        <div
                            class="grid"
                            style=("grid-column", move || format!("{} span", child.1.1))
                        >
                            {move || {
                                if matches!(view_state(), View::Decoded) {
                                    Some(
                                        view! {
                                            <div class="h-6 bg-white grid place-items-center font-mono text-sm">
                                                {child.1.2.clone()}
                                            </div>
                                        },
                                    )
                                } else {
                                    None
                                }
                            }}

                            <div class="h-6 bg-white grid place-items-center">
                                {move || child.1.0}
                            </div>
                        </div>
                    </For>

                </Show>

            </div>
        }
    };

    view! {
        <div class="grid gap-2">
            <div class="text-center">"Instructions"</div>
            <div class="grid grid-cols-3 border-2 border-gray-900 overflow-hidden">
                {[View::Hex, View::Binary, View::Decoded]
                    .map(|x| {
                        view! {
                            <button
                                on:click=move |_| view_state.set(x)
                                style="height: 24px;"
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
                                    View::Hex => "hex",
                                    View::Binary => "binary",
                                    View::Decoded => "decoded",
                                }}

                            </button>
                        }
                    })}

            </div>
            <div
                class="relative grid border-2 border-gray-900 bg-gray-900"
                style="gap: 1px; grid-template-columns: auto auto 415px;"
            >
                <div
                    class="absolute left-0 right-0 ring-4 ring-blue-300 transition-all pointer-events-none"
                    style=move || {
                        format!("height: 52px; top: {}px;", 41 + 50 * ((pc() - start()) / 4))
                    }
                >
                </div>
                <div class="bg-gray-100 py-2 text-center font-medium">"addr"</div>
                <div class="bg-gray-100 py-2 text-center font-medium">"instr"</div>
                <div class="bg-gray-100 py-2 text-center font-medium" style="width: 415;">
                    {move || match view_state() {
                        View::Binary => "binary",
                        View::Decoded => "decoded",
                        View::Hex => "hex",
                    }}

                </div>

                <For
                    each=move || program().into_iter().enumerate()
                    key=|(index, _)| *index
                    children=move |(index, _)| {
                        let value = create_memo(move |_| {
                            program.with(|program| program[index])
                        });
                        let (name, i_type) = code_to_name(value());
                        view! {
                            <div class="bg-white grid place-items-center">
                                {format!("{:02x}", 4 * index)}
                            </div>
                            <div class="bg-white grid place-items-center font-mono">{name}</div>
                            <div>{view_instruction(i_type, value())}</div>
                        }
                    }
                />

            </div>
        </div>
    }
}

#[component]
pub fn Registers(registers: RwSignal<Registers>) -> impl IntoView {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ViewState {
        Bytes,
        U32,
        I32,
    }
    pub fn view_register(word: u32, view_state: ViewState) -> impl IntoView {
        match view_state {
            ViewState::Bytes => view_word(word).into_view(),
            ViewState::U32 => view! {
                <div class="grid place-items-center" style="width: 131px;">
                    {move || format!("{}", word)}
                </div>
            }
            .into_view(),
            ViewState::I32 => view! {
                <div class="grid place-items-center" style="width: 131px;">
                    {move || format!("{}", i32::from_ne_bytes(word.to_ne_bytes()))}
                </div>
            }
            .into_view(),
        }
    }
    let view_state = RwSignal::new(ViewState::Bytes);

    view! {
        <div class="grid gap-2">
            <div class="text-center">"Registers"</div>
            <div class="grid grid-cols-3 border-2 border-gray-900 overflow-hidden">
                {[ViewState::Bytes, ViewState::U32, ViewState::I32]
                    .map(|x| {
                        view! {
                            <button
                                on:click=move |_| view_state.set(x)
                                class=move || {
                                    format!(
                                        "p1 {}",
                                        if x == view_state() {
                                            "bg-gray-900 text-white font-medium"
                                        } else {
                                            "bg-white"
                                        },
                                    )
                                }
                            >

                                {match x {
                                    ViewState::Bytes => "bytes",
                                    ViewState::U32 => "u32",
                                    ViewState::I32 => "i32",
                                }}

                            </button>
                        }
                    })}

            </div>
            <div class="grid grid-cols-2">
                {(0..2)
                    .map(|i| {
                        view! {
                            <div class="grid gap-y-px border-2 border-gray-900 bg-gray-900">
                                <div class="grid grid-cols-[4rem_auto] bg-gray-100 font-medium">
                                    <div class="py-2 text-center border-r-2 border-gray-900 ">
                                        "reg"
                                    </div>
                                    <div class="py-2 text-center">"value"</div>
                                </div>
                                <div class="grid grid-cols-[4rem_auto] bg-white font-mono">
                                    <p class=" text-right px-2 border-r-2 border-gray-900 font-semibold">
                                        {move || if i == 0 { "pc" } else { "-" }}
                                    </p>
                                    {move || {
                                        if i == 0 {
                                            view_register(registers()[PC], view_state()).into_view()
                                        } else {
                                            "".into_view()
                                        }
                                    }}

                                </div>
                                <For
                                    each=move || (0..16)

                                    key=|index| *index
                                    children=move |index| {
                                        let index = index + 16 * i;
                                        let value = create_memo(move |_| {
                                            registers.with(|registers| registers[index])
                                        });
                                        view! {
                                            <div class="grid grid-cols-[4rem_auto] bg-white font-mono">
                                                <p class="text-right px-2 border-r-2 border-gray-900 font-semibold">
                                                    {REGISTER_NAMES[index]}
                                                </p>
                                                {move || view_register(value(), view_state())}
                                            </div>
                                        }
                                    }
                                />

                            </div>
                        }
                    })
                    .collect_view()}

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
        <div class="grid gap-x-px grid-cols-4 place-items-center bg-gray-900">
            <div class="w-8 bg-white text-center">{format!("{a:02x}")}</div>
            <div class="w-8 bg-white text-center">{format!("{b:02x}")}</div>
            <div class="w-8 bg-white text-center">{format!("{c:02x}")}</div>
            <div class="w-8 bg-white text-center">{format!("{d:02x}")}</div>
        </div>
    }
}

#[component]
pub fn Memory(memory: RwSignal<Memory>) -> impl IntoView {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ViewState {
        Bytes,
        U32,
        I32,
    }

    let view_state = RwSignal::new(ViewState::Bytes);
    const BYTES_PER_ROW: usize = 32;
    let n_rows = 8;
    let n_cols = move || {
        BYTES_PER_ROW
            / (match view_state() {
                ViewState::Bytes => 1,
                ViewState::U32 => 4,
                ViewState::I32 => 4,
            })
    };

    view! {
        <div class="grid gap-2 p-4 bg-white ring-1 ring-gray-500/5 shadow-sm">
            <div class="text-center">"RAM"</div>
            <div class="grid grid-cols-3 border-2 border-gray-900 overflow-hidden">
                {[ViewState::Bytes, ViewState::U32, ViewState::I32]
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
                                    ViewState::Bytes => "bytes",
                                    ViewState::U32 => "u32",
                                    ViewState::I32 => "i32",
                                }}

                            </button>
                        }
                    })}

            </div>
            <div
                class="grid gap-px bg-gray-900 border-2 border-gray-900 font-mono"
                style=move || format!("grid-template-columns: 4rem repeat({}, 1fr);", n_cols())
            >
                <span class="py-2 font-sans font-medium bg-gray-100 text-center border-r border-gray-900">
                    "addr"
                </span>
                <span
                    class="py-2 font-sans font-medium bg-gray-100 text-center"
                    style=move || format!("grid-column: span {}", n_cols())
                >
                    "value"
                </span>
                <For
                    each=move || (0..n_rows)

                    key=|row| *row
                    children=move |row| {
                        view! {
                            <span class="bg-white text-right pr-2 font-semibold border-r border-gray-900">
                                {move || format!("{:02x}", BYTES_PER_ROW * row)}
                            </span>
                            <For
                                each=move || (0..n_cols())
                                key=|index| *index
                                children=move |col| {
                                    let value = create_memo(move |_| {
                                        memory
                                            .with(|memory| -> String {
                                                let index = BYTES_PER_ROW * row + col;
                                                match view_state() {
                                                    ViewState::Bytes => format!("{:02x}", memory[index]),
                                                    ViewState::U32 => {
                                                        u32::from_le_bytes(
                                                                memory[index..index + 4].try_into().unwrap(),
                                                            )
                                                            .to_string()
                                                    }
                                                    ViewState::I32 => {
                                                        i32::from_le_bytes(
                                                                memory[index..index + 4].try_into().unwrap(),
                                                            )
                                                            .to_string()
                                                    }
                                                }
                                            })
                                    });
                                    view! {
                                        // note we must create reactive variable here to force render for animation

                                        {move || {
                                            view! {
                                                <span
                                                    class=" text-center transition-all bg-white"
                                                    style="animation: 1200ms appear;"
                                                >
                                                    {value()}
                                                </span>
                                            }
                                        }}
                                    }
                                }
                            />
                        }
                    }
                />

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
