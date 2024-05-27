# RISC-V core

A minimal RV32I RISC-V core implement in Rust. This is just a weekend project, but some tests actually pass!

## Usage

```
trunk serve
```

## Compile programs

```
riscv32-none-elf-gcc -march=rv32i -mabi=ilp32 -c fib.c
riscv32-none-elf-objdump -d fib.o
```

## RISC-V tests

Download and compile tests from https://github.com/riscv/riscv-tests.

Then, run:

```
cargo run
```

to run all tests located at `riscv-tests/isa/rv32ui-p*`. All of them should pass.

## TODO

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
