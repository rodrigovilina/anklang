use crate::{
  parser::{Parse, Unparse},
  Node, Token,
};

#[derive(Debug, PartialEq, Clone)]
pub enum List {
  Single(Box<Node>),
  Multi { first: Box<Node>, rest: Vec<Node> },
}

impl Unparse for List {
  fn unparse(&self) -> String {
    match self {
      Self::Single(node) => format!("({})", node.unparse()),
      Self::Multi { first, rest } => {
        let inner = rest
          .iter()
          .map(Unparse::unparse)
          .collect::<Vec<_>>()
          .join(" ");
        format!("({} {inner})", first.unparse())
      },
    }
  }
}

impl Parse for List {
  fn parse(tokens: &[Token]) -> Option<(Node, &[Token])> {
    if let Some((Token::LP, rest)) = tokens.split_first() {
      let mut depth = 1;
      for (i, token) in rest.iter().enumerate() {
        match token {
          Token::LP => depth += 1,
          Token::RP => depth -= 1,
          _ => (),
        }
        if depth == 0 {
          let (inside, remaining) = rest.split_at(i);
          let mut nodes = Vec::new();
          let mut inner_tokens = inside;
          while !inner_tokens.is_empty() {
            if let Some((node, rest)) = Node::parse(inner_tokens) {
              nodes.push(node);
              inner_tokens = rest;
            } else {
              return None;
            }
          }
          if nodes.len() == 1 {
            return Some((
              Node::List(Box::new(Self::Single(Box::new(nodes[0].clone())))),
              &remaining[1..],
            ));
          }

          let first = Box::new(nodes.remove(0));
          let rest = nodes;
          return Some((
            Node::List(Box::new(Self::Multi { first, rest })),
            &remaining[1..],
          ));
        }
      }
    }
    None
  }
}
