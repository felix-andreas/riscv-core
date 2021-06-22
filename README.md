# RISC-V core

A RISC-V core implement in Rust. This is only a weeked project, but some test actually passed!

* Based on the Python implementation of twitchcore 
    * https://github.com/geohot/twitchcore
    * https://www.youtube.com/watch?v=camQ9QeBY9Q
* Inspired by https://github.com/fintelia/riscv-decode

## Setup

Download and compile tests from https://github.com/riscv/riscv-tests

Tip: Use nix to get a shell with riscv cross compiler
```
nix shell nixpkgs#pkgsCross.riscv64-embedded.buildPackages.gcc 
nix shell nixpkgs#pkgsCross.riscv64-embedded.buildPackages.binutils
```
