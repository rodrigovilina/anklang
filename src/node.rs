use crate::{atom::Atom, list::List, parse::Parse, token::Token, unit::Unit, unparse::Unparse};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Unit,
  Atom(Atom),
  List(List),
}

impl Parse for Node {
  fn parse(tokens: &[Token]) -> Option<(Self, &[Token])> {
    Unit::parse(tokens)
      .or_else(|| Atom::parse(tokens))
      .or_else(|| List::parse(tokens))
  }
}

impl Unparse for Node {
  fn unparse(&self) -> String {
    match self {
      Node::Unit => Unit().unparse(),
      Node::Atom(a) => a.unparse(),
      Node::List(l) => l.unparse(),
    }
  }
}
