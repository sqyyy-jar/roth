use std::{
    env::args,
    fs::{self, File},
    io::{Cursor, Read},
    process::exit,
};

use runtime::VirtualMachine;
use util::read_string_constants;

pub mod bytecode;
pub mod checker;
pub mod compiler;
pub mod parser;
pub mod runtime;
pub mod util;

pub struct Flags {
    pub verify: bool,
    pub prealloc: usize,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            verify: true,
            prealloc: 8,
        }
    }
}

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() < 3 {
        help();
        return;
    }
    match args[1].as_str() {
        "compile" | "c" => {
            if args.len() < 4 {
                help();
                return;
            }
            let mut flags = Flags::default();
            if args.len() > 4 {
                parse_flags(&mut flags, &args[4..]);
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
            let parse_result = parser::parse(source.unwrap().as_str(), &flags);
            if let Err(err) = parse_result {
                println!("Could not parse: {err}");
                return;
            }
            if let Err(err) = compiler::compile(&mut target.unwrap(), &parse_result.unwrap()) {
                println!("Could not compile: {err}");
            };
        }
        "run" | "r" => {
            if args.len() < 3 {
                help();
                return;
            }
            let mut flags = Flags::default();
            if args.len() > 3 {
                parse_flags(&mut flags, &args[3..]);
            }
            let fio = File::open(&args[2]);
            if let Err(err) = fio {
                println!("Could not open file: {err}");
                return;
            }
            let mut fio = fio.unwrap();
            let constants = read_string_constants(&mut fio);
            if let Err(err) = constants {
                println!("Could not read constants: {err}");
                return;
            }
            let mut bytes = Vec::new();
            if let Err(err) = fio.read_to_end(&mut bytes) {
                println!("Could not read file: {err}");
                return;
            };
            let mut vm = VirtualMachine::new(
                &bytes,
                0,
                if flags.verify {
                    let check = checker::check(&bytes);
                    if let Err(err) = check {
                        println!("Invalid bytecode: {err}");
                        return;
                    }
                    let check = check.unwrap();
                    check.0
                } else {
                    4096 * 16
                },
                util::default_panic_handler,
                constants.unwrap(),
                flags,
            );
            vm.execute();
        }
        "interpret" | "i" => {
            if args.len() < 3 {
                help();
                return;
            }
            let mut flags = Flags::default();
            if args.len() > 3 {
                parse_flags(&mut flags, &args[3..]);
            }
            let source = fs::read_to_string(&args[2]);
            if let Err(err) = source {
                println!("Could not read source file: {err}");
                return;
            }
            let parse_result = parser::parse(source.unwrap().as_str(), &flags);
            if let Err(err) = parse_result {
                println!("Could not parse: {err}");
                return;
            }
            let mut bytes = Vec::new();
            if let Err(err) = compiler::compile(&mut bytes, &parse_result.unwrap()) {
                println!("Could not compile: {err}");
                return;
            };
            let mut read = Cursor::new(&bytes);
            let constants = read_string_constants(&mut read);
            if let Err(err) = constants {
                println!("Could not read constants: {err}");
                return;
            }
            let stack_size = if flags.verify {
                let check = checker::check(&bytes[read.position() as usize..]);
                if let Err(err) = check {
                    println!("Invalid bytecode: {err}");
                    return;
                }
                check.unwrap().0.max(4096)
            } else {
                4096 * 16
            };
            let mut vm = VirtualMachine::new(
                &bytes[read.position() as usize..],
                0,
                stack_size,
                util::default_panic_handler,
                constants.unwrap(),
                flags,
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
compile, c    [source file] [target file] [flags]  Compile file to binary
run, r        [file] [flags]                       Run compiled binary
interpret, i  [source file] [flags]                Run file directly

Flags:
-verify             Enable full verification
-noverify           Disable some amount of verification
-prealloc [amount]  Set size of preallocated memory for strings
"#
    );
}

fn parse_flags(flags: &mut Flags, args: &[String]) {
    let mut iter = args.iter();
    while let Some(flag) = iter.next() {
        match flag.as_str() {
            "-verify" => flags.verify = true,
            "-noverify" => flags.verify = false,
            "-prealloc" => {
                let Some(amount) = iter.next() else {
                    help();
                    exit(-1);
                };
                let Ok(amount) = amount.parse() else {
                    help();
                    exit(-1);
                };
                flags.prealloc = amount;
            }
            _ => {}
        }
    }
}
