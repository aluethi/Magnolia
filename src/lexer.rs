use crate::error::Error;

use std::{
    iter,
    str::{self, CharIndices},
};

use crate::token::{
    Token,
    TokenKind,
    Span,
    FloatKind,
    IntegerKind,
};

#[derive(Debug)]
pub struct Lexer<'a> {
    iter: iter::Peekable<str::CharIndices<'a>>,
    source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            iter: source.char_indices().peekable(),
            source,
        }
    }

    fn token(&mut self) -> Result<Option<Token<'a>>, Error> {
        match self.iter.peek() {
            Some((_, c)) => {
                let token = match c {
                    '(' => self.left_paren(),
                    ')' => self.right_paren(),
                    '"' => self.string_literal(),
                    ' ' | '\t' | '\n' | '\r' => self.whitespace(),
                    /*_ => Some(Token::new(TokenKind::Eof, Span { start: *pos as u32, end: (*pos + 1) as u32 })),*/
                    _ => self.reserved(),
                };
                return Ok(token);
            },
            None => return Ok(None),
        }
    }
    
    fn left_paren(&mut self) -> Option<Token<'a>> {
        if let Some((pos, _)) = self.iter.next() {
            return Some(Token::new(TokenKind::LeftParen,
                Span { start: pos as u32, end: (pos + 1) as u32 }));
        }
        None
    }

    fn right_paren(&mut self) -> Option<Token<'a>> {
        if let Some((pos, _)) = self.iter.next() {
            return Some(Token::new(TokenKind::RightParen,
                Span { start: pos as u32, end: (pos + 1) as u32 }));
        }
        None
    }

    fn string_literal(&mut self) -> Option<Token<'a>> {
        let (start, _) = self.iter.peek().cloned().unwrap();
        self.iter.next();
        let mut end = start;
        // search for the closing quote
        while let Some((_, c)) = self.iter.peek().cloned() {
            if c != '"' {
                self.iter.next();
            } else {
                break;
            }
        }
        // consume the closing quote
        self.iter.next();
        if let Some((pos, _)) = self.iter.peek().cloned() {
            end = pos;
        }
        Some(Token::new(TokenKind::String(&self.source[start..end]), 
            Span { start: start as u32, end: end as u32 }))
    }

    fn whitespace(&mut self) -> Option<Token<'a>> {
        let (start, _) = self.iter.peek().cloned().unwrap();
        let mut end = start;
        while let Some((pos, c)) = self.iter.peek().cloned() {
            if Self::is_whitespace(c) {
                self.iter.next();
            } else {
                end = pos;
                break;
            }
        }
        Some(Token::new(TokenKind::Whitespace,
            Span { start: start as u32, end: end as u32 }))
    }

    fn reserved(&mut self) -> Option<Token<'a>> {
        let (start, _) = self.iter.peek().cloned().unwrap();
        self.iter.next();
        let mut end = start;
        while let Some((pos, c)) = self.iter.peek().cloned() {
            if Self::is_legal_char(c) {
                self.iter.next();
            } else {
                end = pos;
                break;
            }
        }

        let reserved = &self.source[start..end];

        if let Some(number) = self.number(reserved) {
            Some(Token::new(number, Span { start: start as u32, end: end as u32 }))
        } else if let Some(keyword) = self.keyword(reserved) {
            Some(keyword)
        } else if let Some(identifier) = self.identifier(reserved) {
            Some(identifier)
        } else {
            Some(Token::new(TokenKind::Reserved(&self.source[start..end]),
                Span { start: start as u32, end: end as u32 }))
        }
    }

    fn number(&mut self, src: &'a str) -> Option<TokenKind<'a>> {
        let (negative, num) = if src.starts_with('-') {
            (true, &src[1..])
        } else if src.starts_with('+') {
            (false, &src[1..])
        } else {
            (false, src)
        };

        if num == "inf" {
            return Some(TokenKind::Float(FloatKind::Inf { src, negative }));
        } else if num == "nan" {
            return Some(TokenKind::Float(FloatKind::Nan { src, negative, value: None }));
        } else if num.starts_with("nan:0x") {
            let value = u64::from_str_radix(&num[6..], 16).ok();
            return Some(TokenKind::Float(FloatKind::Nan { src, negative, value }));
        }

        // Are we dealing with a hex or decimal number?
        let(mut iterator, is_hex, test_valid) = if num.starts_with("0x") {
            (num[2..].char_indices(), true, char::is_ascii_hexdigit as fn(&char) -> bool)
        } else {
            (num.char_indices(), false, char::is_ascii_digit as fn(&char) -> bool)
        };

        // Parse the integral part of the number
        let integral = Self::consume_digits(&mut iterator, test_valid).unwrap();

        let it = iterator.clone().next();
        if it.is_none() { // If there are no more characters, we have a valid integer
            if is_hex {
                return Some(TokenKind::Integer(IntegerKind::Hex { src: integral, negative }));
            } else {
                return Some(TokenKind::Integer(IntegerKind::Decimal { src: integral, negative }));
            }
        } else if let Some((_,'.')) = it { // If there is a decimal point, we have a float
            iterator.next();
        } else { // Otherwise, we have an invalid number
            return None;
        }

        // Parse the fractional part of the number
        let fractional= Self::consume_digits(&mut iterator, test_valid).unwrap();

        // If there is an exponent, we have a float


//        if iterator.clone().next() == Some('e') || iterator.clone().next() == Some('E')
//        || iterator.clone().next() == Some('p') || iterator.clone().next() == Some('P') {
//            let mut exponent: String = String::new();
//            if iterator.clone().next() == Some('-') || iterator.clone().next() == Some('+') {
//                exponent.push(iterator.next().unwrap());
//            }
//            while let Some(c) = iterator.next() {
//                if test_valid(&c) {
//                    exponent.push(c);
//                } else {
//                    break;
//                }
//            }
//        }

        if iterator.clone().next().is_none() {
            return Some(TokenKind::Float(FloatKind::Val { src: num, negative, integral, fractional, exponent: "" }));
        } else {
            //None

            return Some(TokenKind::Float(FloatKind::Val { src: num, negative, integral, fractional, exponent: "" }));
        }
    }

    fn keyword(&mut self, src: &'a str) -> Option<Token<'a>> {
        let (start, _) = self.iter.peek().cloned().unwrap();
        let mut end = start;
        while let Some((pos, c)) = self.iter.peek().cloned() {
            if Self::is_identifier(c) {
                self.iter.next();
            } else {
                end = pos;
                break;
            }
        }
        Some(Token::new(TokenKind::Keyword(&self.source[start..end]), 
            Span { start: start as u32, end: end as u32 }))
    }

    fn identifier(&mut self, src: &'a str) -> Option<Token<'a>> {
        let (start, _) = self.iter.peek().cloned().unwrap();
        self.iter.next();
        let mut end = start;
        while let Some((pos, c)) = self.iter.peek().cloned() {
            if Self::is_identifier(c) {
                self.iter.next();
            } else {
                end = pos;
                break;
            }
        }
        Some(Token::new(TokenKind::Identifier(&self.source[start..end]),
            Span { start: start as u32, end: end as u32 }))
    }

    fn consume_digits(it: &mut CharIndices<'a>, good: fn(&char) -> bool) -> Option<&'a str> {
        let src = it.as_str();
        let (start, _) = it.clone().next()?;
        let mut end = start;
        while let Some((pos, c)) = it.clone().next() {
            if good(&c) {
                end = pos + 1;
                it.next();
            } else {
                break;
            }
        }
        Some(&src[0..end-start])
    }

    fn is_alpha(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        }
    }

    fn is_numeric(c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_numeric(c)
    }

    fn is_identifier(c: char) -> bool {
        Self::is_alphanumeric(c) || c == '_'
    }

    fn is_whitespace(c: char) -> bool {
        match c {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }

    fn is_legal_char(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '-' | '+' | '=' | '<' | '>' | '?' | '/' | '\\' | '|' | ':' | ';' | ',' | '.' | '[' | ']' | '{' | '}' | '_' | '"' => true,
            _ => false,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // transpose() Switches Option<Result<T, E>> to Result<Option<T>, E>
        let token = self.token().transpose();
        //self.iter.next();
        token
    }
}