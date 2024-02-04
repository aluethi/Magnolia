#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self {
            kind,
            span,
        }
    }
}

#[derive(Debug)]
pub struct Span {
    pub start: u32, // inclusive
    pub end: u32, // exclusive
}

#[derive(Debug)]
pub enum TokenKind<'a> {
    // Single char tokens
    LeftParen,
    RightParen,
    
    // Literals
    Identifier(&'a str), // starts with $
    StringLiteral(&'a str),
    IntLiteral(&'a str),
    FloatLiteral(Float<'a>),

    Keyword(&'a str),
    Reserved(&'a str),

    Whitespace,
    Eof,
}

#[derive(Debug)]
pub struct Float<'a> {
    pub src: &'a str,
    pub value: FloatValue,
}

#[derive(Debug)]
pub enum FloatValue {
    Inf {
        negative: bool,
    },
    Nan {
        negative: bool,
        value: Option<u64>,
    },
    Val {
        negative: bool,
        integral: u64,
        fractional: u64,
        exponent: i64,
    }
}