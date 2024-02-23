# Interpreter and x86_64 JIT compiler for the [brainfuck language](https://en.wikipedia.org/wiki/Brainfuck)

Author: [Clément CHAPOT](mailto:clement.chapot@polytechnique.edu)<br>

## Building

Build the project using `make`.

This calls `cargo build --release` and copies the binary from `target/release/` into the project root.

## Usage

`./brainfuck <FILE>` to run the brainfuck program written in `<FILE>` using the optimized JIT compiler

`./brainfuck -n <FILE>` to run the brainfuck program written in `<FILE>` using the unoptimized JIT compiler

`./brainfuck -i <FILE>` to run the brainfuck program written in `<FILE>` using the interpreter

Brainfuck samples can be found in the samples directory. Notably, `./brainfuck samples/mandelbrot.bf` prints the mandelbrot set in the terminal.

## Project structure

### Binary

`/src/bin/brainfuck.rs` uses [clap](https://crates.io/crates/clap) to produce a command-line utility

### Interpreter and (unoptimized) compiler

`/src/lib/{command.rs, program.rs}` provides us with structs to represent brainfuck commands and programs

`/src/lib/interpreter.rs` makes it possible to interpret a brainfuck program

`/src/lib/compiler.rs` makes it possible to compile a brainfuck program JIT and to execute the generated machine code

### Optimized compiler

`/src/lib/{optimized_command.rs, optimized_program.rs}` provides us with structs to represent optimized brainfuck commands (series of `+`/`-` are packed in one `Add` and series of `>`/`<` are packed in one `Move`) which are combined in optimized brainfuck programs

`/src/lib/optimized_compiler.rs` does the same as `compiler.rs` but on an optimized brainfuck program

## JIT compiler optimizations

The JIT compiler packs series of `+`/`-` are into one `Add` and series of `>`/`<` into one `Move`.
