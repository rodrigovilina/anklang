use crate::{l_parens::LParens, lex::Lex, number::Number, r_parens::RParens};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  LP,
  RP,
  Number(i64),
}

impl Lex for Token {
  fn lex(input: &str) -> Option<(Self, &str)> {
    let input = input.trim();
    
    LParens::lex(input)
        .or_else(|| RParens::lex(input))
        .or_else(|| Number::lex(input))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Token::lex("("), Some((Token::LP, "")))
  }

  #[test]
  fn test_2() {
    assert_eq!(Token::lex(")"), Some((Token::RP, "")))
  }

  #[test]
  fn test_3() {
    assert_eq!(Token::lex("0"), Some((Token::Number(0), "")))
  }
}
