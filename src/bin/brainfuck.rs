use brainfuck_jit_compiler::lib::{
    compiler::Compiler, interpreter::Interpreter, optimized_compiler::OptimizedCompiler,
    optimized_interpreter::OptimizedInterpreter,
};
use clap::Parser;
use std::{fs, process};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "File containing the brainfuck program to run")]
    file: String,

    #[arg(short, long, help = "Run using interpreter instead of JIT-compiler")]
    interpret: bool,

    #[arg(short, long, help = "Run without compiler optimizations")]
    no_optimize: bool,
}

fn main() {
    let args = Args::parse();

    let program = match fs::read_to_string(args.file) {
        Ok(program) => program.into(),
        Err(e) => {
            eprintln!("Error reading file: {e}");
            process::exit(1);
        }
    };

    if args.interpret && args.no_optimize {
        Interpreter::new(program, 30_000).run();
    } else if args.interpret {
        match OptimizedInterpreter::new(program.into(), 30_000) {
            Ok(mut interpreter) => {
                interpreter.run();
            }
            Err(i) => {
                eprintln!("Unmatched bracket: {i}-th command in file");
                process::exit(2);
            }
        }
    } else if args.no_optimize {
        Compiler::new(program, 30_000).run();
    } else {
        OptimizedCompiler::new(program.into(), 30_000).run();
    }
}
