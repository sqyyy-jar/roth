use std::{env::args, fs};

use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use state::{process, root_state::RootState, State};
use util::source::CharsSource;

pub mod error;
pub mod path;
pub mod state;
pub mod syntax;
pub mod util;

fn main() {
    let filename = args().nth(1).expect("filename");
    let code = fs::read_to_string(&filename).expect("read file");
    let file = SimpleFile::new(&filename, &code);
    let result = process(
        State::Root(RootState::default()),
        CharsSource::new(code.chars()),
    );
    if let Ok(result) = result {
        println!("{result:?}");
        return;
    }
    let mut diagnostic = Diagnostic::error();
    match result.unwrap_err() {
        error::Error::NewlineInStringLiteral { span } => {
            diagnostic = diagnostic
                .with_message("newline was used in string literal")
                .with_code("E00")
                .with_labels(vec![
                    Label::primary((), span).with_message("this newline is not allowed here")
                ])
                .with_notes(vec!["try to use '\\n' instead".to_string()]);
        }
        error::Error::InvalidEscapeCharacterInString { span } => {
            diagnostic = diagnostic
                .with_message("invalid escape character was used in string literal")
                .with_code("E01")
                .with_labels(vec![
                    Label::primary((), span).with_message("this escape character is not valid")
                ])
                .with_notes(vec![
                    "try to use '\\\\' to not escape characters".to_string()
                ]);
        }
        error::Error::ClosingBracketOutOfContext { span } => {
            diagnostic = diagnostic
                .with_message("closing bracket was used out of context")
                .with_code("E02")
                .with_labels(vec![
                    Label::primary((), span).with_message("this bracket is not allowed here")
                ]);
        }
        error::Error::UnexpectedEndOfSource { span } => {
            diagnostic = diagnostic
                .with_message("end of source was reached unexpectedly")
                .with_code("E03")
                .with_labels(vec![Label::primary((), span).with_message("current token")]);
        }
    }
    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = term::Config::default();
    term::emit(&mut writer.lock(), &config, &file, &diagnostic).expect("emit error");
}
