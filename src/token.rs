use crate::{l_parens::LParens, lex::Lex, r_parens::RParens};

#[derive(Debug, PartialEq)]
pub enum Token {
  LP,
  RP,
}

impl Lex for Token {
  fn lex(input: &str) -> Option<(Self, &str)> {
    let input = input.trim();
    
    LParens::lex(input)
        .or_else(|| RParens::lex(input))
  }
}
