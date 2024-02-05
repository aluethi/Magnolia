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
    String(&'a str),
    Integer(IntegerKind<'a>),
    Float(FloatKind<'a>),

    Keyword(&'a str),
    Reserved(&'a str),

    Whitespace,
    Eof,
}

#[derive(Debug)]
pub enum IntegerKind<'a> {
    Decimal {
        src: &'a str,
        negative: bool,
    },
    Hex {
        src: &'a str,
        negative: bool,
    }
}

#[derive(Debug)]
pub enum FloatKind<'a> {
    Inf {
        src: &'a str,
        negative: bool,
    },
    Nan {
        src: &'a str,
        negative: bool,
        value: Option<u64>,
    },
    Val {
        src: &'a str,
        negative: bool,
        integral: u64,
        fractional: u64,
        exponent: i64,
    }
}