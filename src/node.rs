pub mod atom;
pub mod function;
pub mod list;
mod unit;

use {
  crate::{
    parser::{Parse, Unparse},
    token::Token,
  },
  atom::Atom,
  function::Function,
  list::List,
  unit::Unit,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
  Unit,
  Atom(Atom),
  List(Box<List>),
  Function(Box<Function>),
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
      Self::Unit => Unit.unparse(),
      Self::Atom(atom) => atom.unparse(),
      Self::List(list) => list.unparse(),
      Self::Function(_function) => todo!(),
    }
  }
}
