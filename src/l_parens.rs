use crate::{lex::Lex, unlex::Unlex, Token};

#[derive(Debug, PartialEq, Eq)]
pub struct LParens();

impl Unlex for LParens {
  fn unlex(&self) -> &str {
    "("
  }
}

impl Lex for LParens {
  fn lex(input: &str) -> Option<(Token, &str)> {
    if input.starts_with('(') {
      let mut chars = input.chars();
      chars.next();
      Some((Token::LP, chars.as_str()))
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let result = LParens().unlex();
    assert_eq!(result, "(");
  }

  #[test]
  fn test_2() {
    let result = LParens::lex("(");
    assert_eq!(result, Some((Token::LP, "")));
  }

  #[test]
  fn test_3() {
    let result = LParens::lex("(a");
    assert_eq!(result, Some((Token::LP, "a")));
  }
}
