use crate::{parse::Parse, unparse::Unparse, Node, Token};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Atom {
  Number(i64),
}

impl Unparse for Atom {
  fn unparse(&self) -> String {
    match self {
      Atom::Number(n) => format!("{}", n)
    }
  }
}

impl Parse for Atom {
  fn parse(tokens: &[Token]) -> Option<(Node, &[Token])> {
    match tokens {
      [Token::Number(n), rest @ ..] => Some((Node::Atom(Atom::Number(*n)), rest)),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let result = Atom::Number(0).unparse();
    assert_eq!(result, "0");
  }

  #[test]
  fn test_2() {
    let result: Option<(Node, &[Token])> = Atom::parse(&[]);
    assert_eq!(result, None);
  }

  #[test]
  fn test_3() {
    let tokens: [Token; 1] = [Token::Number(0)];
    let result: Option<(Node, &[Token])> = Atom::parse(&tokens);
    assert_eq!(result, Some((Node::Atom(Atom::Number(0)), &tokens[1..])));
  }

  #[test]
  fn test_4() {
    let tokens: [Token; 2] = [Token::Number(0), Token::LP];
    let result: Option<(Node, &[Token])> = Atom::parse(&tokens);
    assert_eq!(result, Some((Node::Atom(Atom::Number(0)), &tokens[1..])));
  }
}
