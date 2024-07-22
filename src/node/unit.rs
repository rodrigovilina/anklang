use crate::{
  parser::{Parse, Unparse},
  Node, Token,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Unit;

impl Unparse for Unit {
  fn unparse(&self) -> String {
    "()".to_string()
  }
}

impl Parse for Unit {
  fn parse(tokens: &[Token]) -> Option<(Node, &[Token])> {
    match tokens {
      [Token::LP, Token::RP, rest @ ..] => Some((Node::Unit, rest)),
      _ => None,
    }
  }
}

#[cfg(test)]
#[path = "../tests/unit_tests.rs"]
mod unit_tests;
