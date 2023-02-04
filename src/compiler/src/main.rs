use std::{env::args, fs};

use codespan_reporting::{
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use state::{process, root_state::RootState, State};
use util::source::CharsSource;

pub mod compiler;
pub mod error;
pub mod path;
pub mod state;
pub mod syntax;
pub mod util;

fn main() {
    let filename = args().nth(1).expect("filename");
    let code = fs::read_to_string(&filename).expect("read file");
    let file = SimpleFile::new(&filename, &code);
    let result = process(State::Root(RootState::default()), CharsSource::new(&code));
    if let Ok(result) = result {
        println!("{result:#?}");
        return;
    }
    let diagnostic = result.unwrap_err().diagnostic();
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    term::emit(&mut writer.lock(), &config, &file, &diagnostic).expect("emit error");
}
