use riscv_core::run;

fn main() {
    for entry in glob::glob("riscv-tests/isa/rv32ui-p-*").unwrap() {
        let path = entry.unwrap();
        if path.is_dir() || path.extension().is_some() {
            continue;
        }

        println!("ELF file: {:?}", path);
        run(&path, false);
    }
}
