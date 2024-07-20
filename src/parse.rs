use crate::{node::Node, Token};

pub trait Parse {
  fn parse(input: &[Token]) -> Option<(Node, &[Token])>;
}
