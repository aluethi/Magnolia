use crate::error::Error;

use std::{
    iter,
    str,
};

use crate::token::{
    Token,
    TokenKind,
    Span,
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
        Some(Token::new(TokenKind::StringLiteral(&self.source[start..end]), 
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
            Some(number)
        } else if let Some(keyword) = self.keyword(reserved) {
            Some(keyword)
        } else if let Some(identifier) = self.identifier(reserved) {
            Some(identifier)
        } else {
            Some(Token::new(TokenKind::Reserved(&self.source[start..end]),
                Span { start: start as u32, end: end as u32 }))
        }
    }

    fn number(&mut self, src: &'a str) -> Option<Token<'a>> {
        let (negative, num) = if src.starts_with('-') {
            (true, &src[1..])
        } else if src.starts_with('+') {
            (false, &src[1..])
        } else {
            (false, src)
        };

        if(num == "inf" || num == "nan") {
            return Some(Token::new(TokenKind::FloatLiteral(&self.source[0..src.len()]),
                Span { start: 0, end: src.len() as u32 }));
        }
        None
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