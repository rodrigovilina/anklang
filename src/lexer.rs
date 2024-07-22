use crate::Token;

pub trait Lex {
  fn lex(input: &str) -> Option<(Token, &str)>;
}

pub trait Unlex {
  fn unlex(&self) -> &str;
}

#[must_use]
pub fn lex_all(input: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut remainder = input;

  while let Some((token, rest)) = Token::lex(remainder) {
    tokens.push(token);
    remainder = rest;
  }

  tokens
}
