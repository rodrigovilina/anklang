pub mod l_parens;
mod lex;
mod parse;
mod r_parens;
pub mod unit;
pub mod unlex;
pub mod token;
pub mod node;
pub mod unparse;

use {lex::Lex, node::Node, parse::Parse, token::Token};

pub fn lex_all(input: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut remainder = input;

  while let Some((token, rest)) = Token::lex(remainder) {
    tokens.push(token);
    remainder = rest;
  }

  tokens
}

pub fn parse_all(tokens: &[Token]) -> Vec<Node> {
  let mut nodes = Vec::new();
  let mut remainder = tokens;

  while let Some((node, rest)) = Node::parse(remainder) {
    nodes.push(node);
    remainder = rest;
  }

  nodes
}
