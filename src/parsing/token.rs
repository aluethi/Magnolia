use std::fmt::{
    Debug,
    Display,
    Formatter
};
use std::ops::{Index, Range};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default, Hash)]
pub struct Span {
    // inclusive
    pub start: u32,
    // exclusive
    pub end: u32,
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start as usize..span.end as usize
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum TokenKind {
    // Keywords
    Let,
    Fn,
    If,
    Else,
    Return,
    // Literals
    IntLiteral(i32),
    FloatLiteral(f32),
    StringLiteral(String),
    // Identifiers
    Identifier(String),
    // Misc
    Whitespace,
    Comment,
    Newline,
    Eof,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - <{},{}>", self.kind, self.span.start, self.span.end)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}