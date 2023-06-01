# Rusty-Banker by Quang Dang

This is a Rust reimplementation and extension of Professor Tim Richards' Banker and Ledger (Producer/Consumer) project originally written in C++.

## Description
The program will parse the input text file by spawning a number of threads based on the number of file readers and bank workers given in the command line argument. The worker threads will print out the result of the transaction after finishing it.

## Learning Objectives
1. Learn a new language and employ its memory management conventions (ownership and borrowing) to my own project
2. Employ Rust's concurrency and synchronization techniques to prevent deadlocks and data races and ensure thread safety
3. Implement a new functionality not available in the original project

## Installation and Setting Up
You will need to have Rust configured on your local machine before running the program. Rust is installed via the rustup installer, which supports installation on Windows, macOS, and Linux. Follow the rustup installation guidance for your platform and install any extra tools required to build and run Rust programs.

If you are using VSCode, you will also need to install the Rust-Analyzer extension.

After having every installed, clone the repository to your local machine and run the following:

```bash
cargo build
```
Change directory to src:

```bash
cd src
```

Then, you may run the program with the following format 
```bash 
cargo run <num_readers> <num_workers/ledgers> <inputfile.txt>
```
p/s: Please email qdang@umass.edu if you encounter any problems.

## Video Recording and Slides

YouTube presentation and demonstration:https://youtu.be/Cncn-zZLRR4

Slides: https://docs.google.com/presentation/d/1G65yaFyIjGzbfYLfSPn2RrUm4UI_0bVYhso1IovQ7VQ/edit#slide=id.p
