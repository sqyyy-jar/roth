use thiserror::Error;

use crate::syntax::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("NewlineInStringLiteral: {span:?}")]
    NewlineInStringLiteral { span: Span },
    #[error("InvalidEscapeCharacterInString: {span:?}")]
    InvalidEscapeCharacterInString { span: Span },
    #[error("ClosingBracketOutOfContext: {span:?}")]
    ClosingBracketOutOfContext { span: Span },
    #[error("UnexpectedEndOfSource: {span:?}")]
    UnexpectedEndOfSource { span: Span },
}
