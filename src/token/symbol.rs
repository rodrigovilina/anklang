use {
  crate::{
    lexer::{Lex, Unlex},
    Token,
  },
  regex::Regex,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol(String);

impl Unlex for Symbol {
  fn unlex(&self) -> &str {
    &self.0
  }
}

impl Lex for Symbol {
  fn lex(input: &str) -> Option<(Token, &str)> {
    let re = Regex::new(r"^[^\d()\s][^()\s]*").unwrap();
    re.find(input).map(|mat| {
      let symbol = mat.as_str().to_string();
      let rest: &str = &input[mat.end()..];
      (Token::Symbol(symbol), rest)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert_eq!(Symbol("a".to_string()).unlex(), "a");
  }

  #[test]
  fn test_2() {
    assert_eq!(Symbol::lex("a"), Some((Token::Symbol("a".to_string()), "")));
  }
}
