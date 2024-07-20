use yapl::{l_parens::LParens, lex_all, node::Node, parse_all, token::Token, unlex::Unlex, unparse::Unparse};

fn main() {
  let input: &str = "()";
  let tokens: Vec<Token> = lex_all(input);
  let nodes: Vec<Node> = parse_all(&tokens);

  dbg!(tokens);
  dbg!(nodes.clone());
  dbg!(nodes.first().unwrap().unparse());
  dbg!(LParens().unlex());
}
