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

#[test]
fn test_4() {
  test_helper("(123 123)", "(123 123)");
  test_helper("(  123   123  )", "(123 123)");
}

#[test]
fn test_5() {
  test_helper("(())", "()");
  test_helper("((()))", "()");
  test_helper("(()())", "(() ())");
}

#[test]
fn test_addition() {
  test_helper("(+ 1)", "1");
  test_helper("(+ 1 2)", "3");
  test_helper("(+ 1 2 3 4)", "10");
  test_helper("(+)", "0");
}

#[test]
fn test_substraction() {
  test_helper("(- 1)", "-1");
  test_helper("(- -1)", "1");
  test_helper("(- 4 2)", "2");
}

#[test]
fn test_multiplication() {
  test_helper("(* 2)", "2");
  test_helper("(* 2 3)", "6");
  test_helper("(* 1 2 3 4)", "24");
  test_helper("(*)", "1");
}

#[test]
fn test_division() {
  test_helper("(/ 1)", "1");
  test_helper("(/ -1)", "-1");
  test_helper("(/ 4 2)", "2");
}

// #[test]
// fn test_7() {
//   test_helper("(  + 2   (*  3  4)  )", "(+ 2 (* 3 4))");
// }
