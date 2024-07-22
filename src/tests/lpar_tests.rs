use super::*;

#[test]
fn test_1() {
  let result = LParens.unlex();
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
