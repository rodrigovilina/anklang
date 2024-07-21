use super::*;

#[test]
fn test_1() {
  let result = Unit.unparse();
  assert_eq!(result, "()");
}

#[test]
fn test_2() {
  let result: Option<(Node, &[Token],),> = Unit::parse(&[],);
  assert_eq!(result, None);
}

#[test]
fn test_3() {
  let tokens: [Token; 2] = [Token::LP, Token::RP,];
  let result: Option<(Node, &[Token],),> = Unit::parse(&tokens,);
  assert_eq!(result, Some((Node::Unit, &tokens[2..])));
}

#[test]
fn test_4() {
  let tokens = [Token::LP, Token::RP, Token::LP,];
  let result: Option<(Node, &[Token],),> = Unit::parse(&tokens,);
  assert_eq!(result, Some((Node::Unit, &tokens[2..])));
}
