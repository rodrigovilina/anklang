use crate::{atom::Atom, parse::Parse, token::Token, unit::Unit, unparse::Unparse};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Unit,
  Atom(Atom),
}

impl Parse for Node {
  fn parse(tokens: &[Token]) -> Option<(Self, &[Token])> {
    Unit::parse(tokens)
      .or_else(|| Atom::parse(tokens))
  }
}

impl Unparse for Node {
  fn unparse(&self) -> String {
    match self {
      Node::Unit => Unit().unparse(),
      Node::Atom(a) => a.unparse(),
    }
  }
}
