use brainfuck_jit_compiler::lib::interpreter::Interpreter;
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
        Ok(program) => program.into(),
        Err(e) => {
            eprintln!("Error reading file: {e}");
            process::exit(1);
        }
    };

    match Interpreter::new(program, 30_000) {
        Ok(mut machine) => {
            machine.run();
        }
        Err(i) => {
            eprintln!("Unmatched bracket: {i}-th command in file");
            process::exit(2);
        }
    }
}
