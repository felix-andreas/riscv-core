# RISC-V core

A minimal RV32I RISC-V core implement in Rust. This is just a weekend project, but all `rv32ui` tests actually pass!

## RISC-V tests

Download and compile tests from https://github.com/riscv/riscv-tests. Or, run:

```
nix build .#riscv-tests
```

Then, run:

```
cargo run -p riscv --example run_tests -- <path/to/tests>
```

where `<path/to/tests>` should be either `riscv-tests/isa` or `result`, depending on if you compilied the tests manually or with Nix. The command runs all `rv32ui-p*` tests. All of them should pass.

## Visualization (WIP)

Currently working on a visualization. You can see a work in progress version at: https://riscv.felixandreas.me/

### Usage

```
cd web
trunk serve
```

### Compile programs

```
riscv32-none-elf-gcc -march=rv32i -mabi=ilp32 -c fib.c
riscv32-none-elf-objdump -d fib.o
```

### TODO

* Fix program
* animate change
* Visualize rs1, rs2, rd
* Better error message
* Show different numbers on hover
* Implement clock (with little animation cake) + indicator if running or not
* Refactor code into two crates
* Set font (don't just use system font)

## References

* [The RISC-V Instruction Set Manual](https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf)
* [twitchcore](https://github.com/geohot/twitchcore)
* [twitchcore lesson](https://www.youtube.com/watch?v=camQ9QeBY9Q)
