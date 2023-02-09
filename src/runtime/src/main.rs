use std::{env::args, fs::File};

use roth_bytecode::format::BinarySection;

fn main() {
    let mut bin_file = File::open(args().nth(1).unwrap()).unwrap();
    let binary = roth_bytecode::format::Binary::read(&mut bin_file).unwrap();
    if binary.sections.is_empty() {
        panic!("No section");
    }
    let mut strings_index = -1;
    let mut functions_index = -1;
    let mut modules_index = -1;
    let mut imports_index = -1;
    for (i, section) in binary.sections.iter().enumerate() {
        match section {
            BinarySection::Strings { strings: _ } => {
                if strings_index != -1 {
                    panic!("More than one strings section");
                }
                strings_index = i as isize;
            }
            BinarySection::Functions { functions: _ } => {
                if functions_index != -1 {
                    panic!("More than one functions section");
                }
                functions_index = i as isize;
            }
            BinarySection::Modules { modules: _ } => {
                if modules_index != -1 {
                    panic!("More than one modules section");
                }
                modules_index = i as isize;
            }
            BinarySection::Imports { imports: _ } => {
                if imports_index != -1 {
                    panic!("More than one imports section");
                }
                imports_index = i as isize;
            }
        }
    }
}
