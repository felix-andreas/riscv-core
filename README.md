# RISC-V core

* based on Python implementation of twitchcore 
    * https://github.com/geohot/twitchcore
    * https://www.youtube.com/watch?v=camQ9QeBY9Q
* inspired by https://github.com/fintelia/riscv-decode

## Setup

download and compile tests from https://github.com/riscv/riscv-tests

get shell with riscv cross compiler using nix 
```
nix shell nixpkgs#pkgsCross.riscv64-embedded.buildPackages.gcc 
nix shell nixpkgs#pkgsCross.riscv64-embedded.buildPackages.binutils
```
