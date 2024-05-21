# RISC-V core

A minimal RV32I RISC-V core implement in Rust. This is just a weekend project, but some tests actually pass!

* Based on the Python implementation of twitchcore 
    * https://github.com/geohot/twitchcore
    * https://www.youtube.com/watch?v=camQ9QeBY9Q
* Inspired by https://github.com/fintelia/riscv-decode

## Setup


## RISC-V tests

Download and compile tests from https://github.com/riscv/riscv-tests.

Then, run:

```
cargo run
```

to run all tests located at `riscv-tests/isa/rv32ui-p*`. All of them should pass.
