# Rusty-Banker by Quang Dang

This is my final project for CS 377: Operating Systems. It is a Rust reimplementation and extension of Professor Tim Richards' Banker and Ledger project originally written in C++.

This project is intended to demonstrate my knowledge of concurrency and synchronization regardless of languages. 

## Installation and Setting Up
You will need to have Rust configured on your local machine before running the program. Rust is installed via the rustup installer, which supports installation on Windows, macOS, and Linux. Follow the rustup installation guidance for your platform and install any extra tools required to build and run Rust programs.

If you are using VSCode, you will also need to install the Rust-Analyzer extension.

After having every installed, clone the repository to your local machine and run the following:

```bash
cargo build
```

Then, you may run the program with the following format 
```bash 
cargo run <num_readers> <num_workers/ledgers> <inputfile.txt>
```
p/s: Please email qdang@umass.edu if you encounter any problems.

##Video Recording and Slides

YouTube presentation and demonstration: 

Slides: https://docs.google.com/presentation/d/1G65yaFyIjGzbfYLfSPn2RrUm4UI_0bVYhso1IovQ7VQ/edit#slide=id.p
