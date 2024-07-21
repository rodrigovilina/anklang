use {
  crate::{lex::Lex, unlex::Unlex, Token},
  regex::Regex,
};

#[derive(Debug, PartialEq, Eq,)]
pub struct Number(String,);

impl Unlex for Number {
  fn unlex(&self,) -> &str {
    &self.0
  }
}

impl Lex for Number {
  fn lex(input: &str,) -> Option<(Token, &str,),> {
    let re: Regex = Regex::new(r"^-?(0|[1-9]\d*)",).unwrap();
    if let Some(mat,) = re.find(input,) {
      let number: i64 = mat.as_str().parse().unwrap();
      let rest: &str = &input[mat.end()..];
      Some((Token::Number(number,), rest,),)
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
    assert_eq!(Number("0".to_string()).unlex(), "0")
  }

  #[test]
  fn test_2() {
    assert_eq!(Number::lex("0"), Some((Token::Number(0), "")))
  }
}
