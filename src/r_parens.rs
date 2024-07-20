use {
  crate::{lex::Lex, unlex::Unlex, Token},
  std::str::Chars,
};

#[derive(Debug, PartialEq, Eq)]
pub struct RParens();

impl Unlex for RParens {
  fn unlex(&self) -> &str {
    ")"
  }
}

impl Lex for RParens {
  fn lex(input: &str) -> Option<(Token, &str)> {
    if input.starts_with(')') {
      let mut chars: Chars = input.chars();
      chars.next();
      Some((Token::RP, chars.as_str()))
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
    let result = RParens().unlex();
    assert_eq!(result, ")");
  }

  #[test]
  fn test_2() {
    let result = RParens::lex(")");
    assert_eq!(result, Some((Token::RP, "")));
  }

  #[test]
  fn test_3() {
    let result = RParens::lex(")a");
    assert_eq!(result, Some((Token::RP, "a")));
  }
}
