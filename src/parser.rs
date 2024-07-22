use crate::{node::Node, Token};

pub trait Parse {
  fn parse(input: &[Token]) -> Option<(Node, &[Token])>;
}

pub trait Unparse {
  fn unparse(&self) -> String;
}

#[must_use]
pub fn parse_all(tokens: &[Token]) -> Vec<Node> {
  let mut nodes = Vec::new();
  let mut remainder = tokens;

  while let Some((node, rest)) = Node::parse(remainder) {
    nodes.push(node);
    remainder = rest;
  }

  nodes
}
