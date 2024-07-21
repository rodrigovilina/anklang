use crate::{parse::Parse, unparse::Unparse, Node, Token};

#[derive(Debug, PartialEq, Clone,)]
pub struct List(pub Vec<Node,>,);

impl Unparse for List {
  fn unparse(&self,) -> String {
    let inner = self
      .0
      .iter()
      .map(|node| node.unparse(),)
      .collect::<Vec<_,>>()
      .join(" ",);
    format!("({})", inner)
  }
}

impl Parse for List {
  fn parse(tokens: &[Token],) -> Option<(Node, &[Token],),> {
    if let Some((Token::LP, rest,),) = tokens.split_first() {
      let mut depth = 1;
      for (i, token,) in rest.iter().enumerate() {
        match token {
          Token::LP => depth += 1,
          Token::RP => depth -= 1,
          _ => (),
        }
        if depth == 0 {
          let (inside, remaining,) = rest.split_at(i,);
          let mut nodes = Vec::new();
          let mut inner_tokens = inside;
          while !inner_tokens.is_empty() {
            if let Some((node, rest,),) = Node::parse(inner_tokens,) {
              nodes.push(node,);
              inner_tokens = rest;
            } else {
              return None;
            }
          }
          return Some((Node::List(Self(nodes,),), &remaining[1..],),);
        }
      }
    }
    None
  }
}
