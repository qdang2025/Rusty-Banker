# Rusty-Banker

This is my final project for CS 377: Operating Systems. It is a Rust reimplementation and extension of Professor Tim Richards' Banker and Ledger project originally written in C++.

This project is intended to demonstrate my knowledge of concurrency and synchronization regardless of languages. 

## Installation and Setting Up

Clone the repository to your local machine and run the following:

```bash
cargo build
```

Then, you may run the program with the following format 
```bash 
cargo run <num_readers> <num_workers/ledgers> <inputfile.txt>
```

