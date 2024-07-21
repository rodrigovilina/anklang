pub mod atom;
mod unit;

use {
  crate::{list::List, parse::Parse, token::Token, unparse::Unparse},
  atom::Atom,
  unit::Unit,
};

#[derive(Debug, PartialEq, Clone,)]
pub enum Node {
  Unit,
  Atom(Atom,),
  List(List,),
}

impl Parse for Node {
  fn parse(tokens: &[Token],) -> Option<(Self, &[Token],),> {
    Unit::parse(tokens,)
      .or_else(|| Atom::parse(tokens,),)
      .or_else(|| List::parse(tokens,),)
  }
}

impl Unparse for Node {
  fn unparse(&self,) -> String {
    match self {
      Self::Unit => Unit.unparse(),
      Self::Atom(atom,) => atom.unparse(),
      Self::List(list,) => list.unparse(),
    }
  }
}
