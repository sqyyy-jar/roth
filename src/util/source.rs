use std::{iter::Peekable, str::Chars};

pub trait Source {
    fn has_next(&mut self) -> bool;

    fn index(&mut self) -> usize;

    fn peek(&mut self) -> Option<char>;

    fn advance(&mut self);

    fn consume_whitespace(&mut self) {
        while self.has_next() {
            if !self.peek().unwrap().is_whitespace() {
                break;
            }
            self.advance();
        }
    }
}

pub struct CharsSource<'a> {
    index: usize,
    chars: Peekable<Chars<'a>>,
}

impl<'a> CharsSource<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            index: 0,
            chars: chars.peekable(),
        }
    }
}

impl Source for CharsSource<'_> {
    fn has_next(&mut self) -> bool {
        self.chars.peek().is_some()
    }

    fn index(&mut self) -> usize {
        self.index
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }

    fn advance(&mut self) {
        let c = self.chars.next().unwrap();
        self.index += c.len_utf8();
    }
}
