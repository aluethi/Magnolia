pub enum Token {
    // Keywords
    Module,
    Func,
    Param,
    Result,
    Local,
    Global,
    Table,
    Memory,
    Export,
    Import,
    Type,
    Start,
    Elem,
    Data,
    Offset,
    Mut,
    If,
    Then,
    Else,
    End,
    // Types
    I32,
    I64,
    F32,
    F64,
    V128,
    Funcref,
    Externref,
    // Symbols
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Colon,
    Equal,
    Minus,
    Plus,
    Star,
    Slash,
    Percent,
    Exclamation,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Dollar,
    Hash,
    At,
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

pub struct Tokenizer<'a> {
    pub input: &'a str,
    pub pos: usize,
    pub line: usize,
    pub col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new (input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.input.chars().nth(self.pos);
        if let Some(c) = c {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        c
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    pub fn eof(&self) -> bool {
        self.peek().is_none()
    }

    pub fn tokenize<'a>(&'a mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while !self.eof() {
            let token = self.next_token();
            tokens.push(token);
        }
        tokens
    }

    pub fn is_whitespace(&self, c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    pub fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn is_hex_digit(&self, c: char) -> bool {
        self.is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
    }

    pub fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    pub fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    pub fn is_symbol(&self, c: char) -> bool {
        match c {
            '(' | ')' | '{' | '}' | '[' | ']' | '<' | '>' | ',' | ';' | ':' | '=' | '-' | '+' | '*' | '/' | '%' | '!' | '&' | '|' | '^' | '~' | '$' | '#' | '@' | '\'' | '"' => true,
            _ => false,
        }
    }
}