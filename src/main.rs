use std::{
    env::args,
    fs::{self, File},
    slice,
};

use runtime::VirtualMachine;

pub mod bytecode;
pub mod checker;
pub mod compiler;
pub mod parser;
pub mod runtime;
pub mod util;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() < 3 {
        help();
        return;
    }
    match args[1].as_str() {
        "compile" | "c" => {
            if args.len() != 4 {
                help();
                return;
            }
            let source = fs::read_to_string(&args[2]);
            if let Err(err) = source {
                println!("Could not read source file: {err}");
                return;
            }
            let target = File::create(&args[3]);
            if let Err(err) = target {
                println!("Could not create target file: {err}");
                return;
            }
            let parse_result = parser::parse(source.unwrap().as_str());
            if let Err(err) = parse_result {
                println!("Could not parse: {err}");
                return;
            }
            if let Err(err) = compiler::compile(&mut target.unwrap(), &parse_result.unwrap()) {
                println!("Could not compile: {err}");
            };
        }
        "run" | "r" => {
            if args.len() != 3 {
                help();
                return;
            }
            let bytes = fs::read(&args[2]);
            if let Err(err) = bytes {
                println!("Could not read file: {err}");
                return;
            }
            let bytes = bytes.unwrap();
            let check = checker::check(&bytes);
            if let Err(err) = check {
                println!("Invalid bytecode: {err}");
                return;
            }
            let check = check.unwrap();
            let mut vm = VirtualMachine::new(
                unsafe { slice::from_raw_parts(bytes.as_ptr(), bytes.len()) },
                0,
                check.0,
                util::default_panic_handler,
            );
            vm.execute();
        }
        "interpret" | "i" => {
            if args.len() != 3 {
                help();
                return;
            }
            let source = fs::read_to_string(&args[2]);
            if let Err(err) = source {
                println!("Could not read source file: {err}");
                return;
            }
            let mut bytes = Vec::new();
            let parse_result = parser::parse(source.unwrap().as_str());
            if let Err(err) = parse_result {
                println!("Could not parse: {err}");
                return;
            }
            if let Err(err) = compiler::compile(&mut bytes, &parse_result.unwrap()) {
                println!("Could not compile: {err}");
                return;
            };
            let check = checker::check(&bytes);
            if let Err(err) = check {
                println!("Invalid bytecode: {err}");
                return;
            }
            let check = check.unwrap();
            let mut vm = VirtualMachine::new(
                unsafe { slice::from_raw_parts(bytes.as_ptr(), bytes.len()) },
                0,
                check.0,
                util::default_panic_handler,
            );
            vm.execute();
        }
        _ => {
            help();
        }
    }
}

fn help() {
    println!(
        r#"Subcommands:
compile, c    [source file] [target file]  Compile file to binary
run, r        [file]                       Run compiled binary
interpret, i  [source file]                Run file directly
"#
    );
}
