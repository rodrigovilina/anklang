use {
  crate::{
    lexer::{Lex, Unlex},
    Token,
  },
  std::str::Chars,
};

#[derive(Debug, PartialEq, Eq)]
pub struct RParens;

impl Unlex for RParens {
  fn unlex(&self) -> &str {
    ")"
  }
}

impl Lex for RParens {
  fn lex(input: &str) -> Option<(Token, &str)> {
    if input.starts_with(')') {
      let mut chars: Chars = input.chars();
      chars.next();
      Some((Token::RP, chars.as_str()))
    } else {
      None
    }
  }
}

#[cfg(test)]
#[path = "../tests/rpar_tests.rs"]
mod rpar_tests;
