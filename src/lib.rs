#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::expect_used)]
#![deny(clippy::min_ident_chars)]
#![deny(clippy::panic)]
// #![warn(clippy::unwrap_used)]
//
// #![deny(clippy::restriction)]
// #![allow(clippy::implicit_return)]
// #![allow(clippy::missing_docs_in_private_items)]

pub mod env;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod token;

use {node::Node, token::Token};

#[cfg(test)]
mod tests {
  use {
    super::*,
    lexer::lex_all,
    parser::{parse_all, Unparse},
  };

  fn test_helper(input: &str, output: &str) {
    let tokens = lex_all(input);
    let nodes = parse_all(&tokens);
    let node = nodes.first().unwrap().clone();
    assert_eq!(node.unparse(), output);
  }

  #[test]
  fn test_4() {
    test_helper("(123 123)", "(123 123)");
    test_helper("(  123   123  )", "(123 123)");
  }

  #[test]
  fn test_5() {
    test_helper("(())", "(())");
    test_helper("((()))", "((()))");
    test_helper("(()())", "(() ())");
  }

  #[test]
  fn test_6() {
    test_helper("(  + 2   (*  3  4)  )", "(+ 2 (* 3 4))");
  }
}
