use super::*;

#[test]
fn test_1() {
  let result = RParens.unlex();
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
