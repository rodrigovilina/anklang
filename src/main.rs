use {
  std::io::{self, BufRead, Write},
  yapl::{
    env::Env,
    lexer::lex_all,
    node::Node,
    parser::{parse_all, Unparse},
  },
};

fn main() {
  let mut env = Env::new();

  loop {
    print(&env.eval(&read()));
  }
}

fn read() -> Node {
  let mut buffer = String::with_capacity(2048);
  let mut stdin = io::stdin().lock();

  print!("|> ");
  io::stdout().flush().unwrap();

  let read_result = stdin.read_line(&mut buffer);

  match read_result {
    Ok(0) => {},
    Ok(_) => {},
    Err(_) => {},
  };

  let tokens = lex_all(&buffer);
  // dbg!(tokens.clone());
  let nodes = parse_all(&tokens);
  // dbg!(nodes.clone());
  nodes.first().unwrap().clone()
}

fn print(node: &Node) {
  let output = node.unparse();
  println!("-> {}", output);
}
