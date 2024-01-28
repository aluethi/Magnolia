use crate::parsing::token::TokenKind;

pub struct Rule {
    pub kind: TokenKind,
    pub matches: fn(&str) -> Option<u32>,
}

pub mod rules_wat;
pub use rules_wat::get_rules as get_rules_wat;