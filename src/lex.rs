use crate::Token;

pub trait Lex {
  fn lex(input: &str) -> Option<(Token, &str)>;
}
