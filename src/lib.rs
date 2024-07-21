#![deny(clippy::complexity)]
#![deny(clippy::expect_used)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
#![deny(clippy::perf)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::min_ident_chars)]
// #![warn(clippy::unwrap_used)]
// #![warn(clippy::pedantic)]
//
// #![deny(clippy::restriction)]
// #![allow(clippy::implicit_return)]
// #![allow(clippy::missing_docs_in_private_items)]

pub mod env;
mod lex;
mod list;
pub mod node;
mod parse;
pub mod token;
pub mod unlex;
pub mod unparse;

use {lex::Lex, node::Node, parse::Parse, token::Token};

pub fn lex_all(input: &str,) -> Vec<Token,> {
  let mut tokens = Vec::new();
  let mut remainder = input;

  while let Some((token, rest,),) = Token::lex(remainder,) {
    tokens.push(token,);
    remainder = rest;
  }

  tokens
}

pub fn parse_all(tokens: &[Token],) -> Vec<Node,> {
  let mut nodes = Vec::new();
  let mut remainder = tokens;

  while let Some((node, rest,),) = Node::parse(remainder,) {
    nodes.push(node,);
    remainder = rest;
  }

  nodes
}

#[cfg(test)]
mod tests {
  use {super::*, unparse::Unparse};

  fn test_helper(input: &str, output: &str,) {
    let tokens = lex_all(input,);
    let nodes = parse_all(&tokens,);
    let node = nodes.first().unwrap().clone();
    assert_eq!(node.unparse(), output)
  }

  #[test]
  fn test_0() {
    test_helper("()", "()",);
    test_helper(" ()", "()",);
    test_helper("() ", "()",);
    test_helper(" () ", "()",);
    test_helper(" ( ) ", "()",);
  }

  #[test]
  fn test_1() {
    test_helper("123", "123",);
    test_helper(" 123", "123",);
    test_helper("123 ", "123",);
    test_helper(" 123 ", "123",);
  }

  #[test]
  fn test_2() {
    test_helper("abc", "abc",);
    test_helper(" abc", "abc",);
    test_helper("abc ", "abc",);
    test_helper(" abc ", "abc",);
  }

  #[test]
  fn test_3() {
    test_helper("(123)", "(123)",);
    test_helper("( 123 )", "(123)",);
  }

  #[test]
  fn test_4() {
    test_helper("(123 123)", "(123 123)",);
    test_helper("(  123   123  )", "(123 123)",);
  }

  #[test]
  fn test_5() {
    test_helper("(())", "(())",);
    test_helper("((()))", "((()))",);
    test_helper("(()())", "(() ())",);
  }

  #[test]
  fn test_6() {
    test_helper("(  + 2   (*  3  4)  )", "(+ 2 (* 3 4))",)
  }
}
