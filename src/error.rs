use codespan_reporting::diagnostic::{Diagnostic, Label};
use thiserror::Error;

use crate::syntax::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("NewlineInStringLiteral: {span:?}")]
    NewlineInStringLiteral { span: Span },
    #[error("InvalidEscapeCharacterInString: {span:?}")]
    InvalidEscapeCharacterInString { span: Span },
    #[error("FunctionNameStartingWithNumber: {span:?}")]
    FunctionNameStartingWithNumber { span: Span },
    #[error("FunctionNameIsKeyword: {span:?}")]
    FunctionNameIsKeyword { span: Span },
    #[error("ClosingBracketOutOfContext: {span:?}")]
    ClosingBracketOutOfContext { span: Span },
    #[error("OpeningBracketOutOfContext: {span:?}")]
    OpeningBracketOutOfContext { span: Span },
    #[error("UnexpectedEndOfSource: {span:?}")]
    UnexpectedEndOfSource { span: Span },
    #[error("UnexpectedCharacter: {span:?}")]
    UnexpectedCharacter { span: Span },
    #[error("UnexpectedToken: {span:?}")]
    UnexpectedToken { span: Span },
    #[error("MissingWhitespaceBetweenTokens: {span:?}")]
    MissingWhitespaceBetweenTokens { span: Span },
}

impl Error {
    pub fn diagnostic(&self) -> Diagnostic<()> {
        let diagnostic = Diagnostic::error();
        match self {
            Self::NewlineInStringLiteral { span } => diagnostic
                .with_message("newline was used in string literal")
                .with_code("E00")
                .with_labels(vec![Label::primary((), span.clone())
                    .with_message("this newline is not allowed here")])
                .with_notes(vec!["try to use '\\n' instead".to_string()]),
            Self::InvalidEscapeCharacterInString { span } => diagnostic
                .with_message("invalid escape character was used in string literal")
                .with_code("E01")
                .with_labels(vec![Label::primary((), span.clone())
                    .with_message("this escape character is not valid")])
                .with_notes(vec![
                    "try to use '\\\\' to not escape characters".to_string()
                ]),
            Self::FunctionNameStartingWithNumber { span } => diagnostic
                .with_message("invalid function name was used in function declaration")
                .with_code("E02")
                .with_labels(vec![Label::primary((), span.clone())
                    .with_message("this cannot start with a number")])
                .with_notes(vec![
                    "try to put something in front of the number".to_string()
                ]),
            Self::FunctionNameIsKeyword { span } => diagnostic
                .with_message("invalid function name was used in function declaration")
                .with_code("E03")
                .with_labels(vec![Label::primary((), span.clone())
                    .with_message("this is already used by a keyword")])
                .with_notes(vec!["try to use something different".to_string()]),
            Self::ClosingBracketOutOfContext { span } => diagnostic
                .with_message("closing bracket was used out of context")
                .with_code("E04")
                .with_labels(vec![Label::primary((), span.clone())
                    .with_message("this bracket is not allowed here")]),
            Self::OpeningBracketOutOfContext { span } => diagnostic
                .with_message("opening bracket was used out of context")
                .with_code("E05")
                .with_labels(vec![Label::primary((), span.clone())
                    .with_message("this bracket is not allowed here")]),
            Self::UnexpectedEndOfSource { span } => diagnostic
                .with_message("end of source was reached unexpectedly")
                .with_code("E06")
                .with_labels(vec![
                    Label::primary((), span.clone()).with_message("end of source")
                ]),
            Self::UnexpectedCharacter { span } => diagnostic
                .with_message("unexpected character in source")
                .with_code("E07")
                .with_labels(vec![
                    Label::primary((), span.clone()).with_message("unexpected character")
                ]),
            Self::UnexpectedToken { span } => diagnostic
                .with_message("unexpected token in source")
                .with_code("E08")
                .with_labels(vec![
                    Label::primary((), span.clone()).with_message("this token is not allowed here")
                ]),
            Self::MissingWhitespaceBetweenTokens { span } => diagnostic
                .with_message("missing whitespace between tokens")
                .with_code("E09")
                .with_labels(vec![
                    Label::primary((), span.clone()).with_message("this is not allowed")
                ])
                .with_notes(vec!["try to use put a space here".to_string()]),
        }
    }
}
