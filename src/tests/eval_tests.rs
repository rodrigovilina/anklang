use super::*;

fn test_helper(input: &str, expected: &str) {
  let mut env = Env::new();
  let result = env.parse_eval_unparse(input);
  assert_eq!(result, expected);
}

#[test]
fn test_0() {
  test_helper("()", "()");
  test_helper(" ()", "()");
  test_helper("() ", "()");
  test_helper(" () ", "()");
  test_helper(" ( ) ", "()");
}

#[test]
fn test_1() {
  test_helper("123", "123");
  test_helper(" 123", "123");
  test_helper("123 ", "123");
  test_helper(" 123 ", "123");
}

// #[test]
// fn test_2() {
//   test_helper("abc", "abc",);
//   test_helper(" abc", "abc",);
//   test_helper("abc ", "abc",);
//   test_helper(" abc ", "abc",);
// }

#[test]
fn test_3() {
  test_helper("(123)", "123");
  test_helper("( 123 )", "123");
}
