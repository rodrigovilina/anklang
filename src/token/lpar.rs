use crate::{
  lexer::{Lex, Unlex},
  Token,
};

#[derive(Debug, PartialEq, Eq)]
pub struct LParens;

impl Unlex for LParens {
  fn unlex(&self) -> &str {
    "("
  }
}

impl Lex for LParens {
  fn lex(input: &str) -> Option<(Token, &str)> {
    if input.starts_with('(') {
      let mut chars = input.chars();
      chars.next();
      Some((Token::LP, chars.as_str()))
    } else {
      None
    }
  }
}

#[cfg(test)]
#[path = "../tests/lpar_tests.rs"]
mod lpar_tests;
