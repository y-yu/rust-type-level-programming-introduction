Type level Programming in Rust
=====================================

[![CI](https://github.com/y-yu/rust-type-level-programming-introduction/actions/workflows/ci.yml/badge.svg)](https://github.com/y-yu/rust-type-level-programming-introduction/actions/workflows/ci.yml)

This is a working example of type level programming in Rust. 

- Type level data
    - `TBool`
    - `TNat`
    - `HList`
- Type level operations
    - `TAnd`, `TOr` for `TBool`
    - `TAdd`, `TSub`, `TEqaul` for `TNat`
    - `TContains<N: TNat>` for `HList`
