use {
    riscv::{step, Memory, Registers, MEMORY_SIZE, MEMORY_START, PC},
    std::{fs::File, io::Read, path::Path},
    xmas_elf::program::SegmentData,
};

fn main() {
    for entry in glob::glob(
        Path::new(
            std::env::args()
                .nth(1)
                .as_deref()
                .unwrap_or("riscv-tests/isa"),
        )
        .join("rv32ui-p-*")
        .to_str()
        .unwrap(),
    )
    .unwrap()
    {
        let path = entry.unwrap();
        if path.is_dir() || path.extension().is_some() {
            continue;
        }

        println!("ELF file: {:?}", path);
        run(&path, false);
    }
}

pub fn run(path: &std::path::Path, verbose: bool) {
    let mut memory: Memory = [0; MEMORY_SIZE];
    let mut registers: Registers = [0; 33];
    load_elf(&mut memory, path);
    registers[PC] = MEMORY_START as u32;

    if verbose {
        println!(
            "{:4} {:8} {:8} {:0}",
            "STEP", "ADDRESS", "CODE", "INSTRUCTION"
        );
    }

    for i in 0.. {
        if verbose {
            let pc = registers[PC];
            let code = riscv::load_word(&memory, pc).unwrap();
            let instruction = riscv::decode(code).unwrap();

            // Uncomment to dump registers for range of instructions
            // if (0x80000198..=0x800001a8).contains(&pc) {
            //     dump_registers(&registers);
            // }

            println!("{:4} {:8x} {:08x} {:?}", i, pc, code, &instruction);
        }

        let done = step(&mut registers, &mut memory).unwrap();
        if done {
            println!("Test succeeded!");
            break;
        }
    }
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
        if let Ok(SegmentData::Undefined(data)) = program_header.get_data(&elf_file) {
            memory[address..address + data.len()].copy_from_slice(data);
        } else {
            panic!("this should panic")
        }
    }
}
