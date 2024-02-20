use brainfuck_jit_compiler::lib::interpreter::{BfMachine, Program};
use clap::{arg, command, value_parser};
use std::{fs, process};

fn main() {
    let matches = command!()
        .arg(
            arg!([FILE] "File containing the brainfuck program to interpret")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let filename = matches.get_one::<String>("FILE").unwrap();

    let program = match fs::read_to_string(filename) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    let program: Program = match Program::try_from(program) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            process::exit(2);
        }
    };

    let mut machine = BfMachine::new(program, 30_000);

    machine.run();
}
