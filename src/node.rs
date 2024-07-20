use crate::{parse::Parse, token::Token, unit::Unit, unparse::Unparse};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Unit,
}

impl Parse for Node {
  fn parse(tokens: &[Token]) -> Option<(Self, &[Token])> {
    Unit::parse(tokens)
  }
}

impl Unparse for Node {
  fn unparse(&self) -> &str {
    match self {
      Node::Unit => Unit().unparse(),
    }
  }
}
