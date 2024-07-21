use {
  std::io::{self, BufRead, Write},
  yapl::{
    atom::Atom,
    env::{Env, Function},
    lex_all,
    node::Node,
    parse_all,
    unparse::Unparse,
  },
};

fn main() {
  let mut env = Env::new();

  let add = Function(Box::new(|args: &[Node]| -> Node {
    let mut sum = 0;
    for arg in args {
      if let Node::Atom(Atom::Number(n,),) = arg {
        sum += n;
      }
    }
    Node::Atom(Atom::Number(sum,),)
  },),);

  env.define("+".to_string(), add,);

  loop {
    print(&env.eval(&read(),),);
  }
}

fn read() -> Node {
  let mut buffer = String::with_capacity(2048,);
  let mut stdin = io::stdin().lock();

  print!("|> ");
  io::stdout().flush().unwrap();

  let read_result = stdin.read_line(&mut buffer,);

  match read_result {
    Ok(0,) => {},
    Ok(_,) => {},
    Err(_,) => {},
  };

  let tokens = lex_all(&buffer,);
  // dbg!(tokens.clone());
  let nodes = parse_all(&tokens,);
  // dbg!(nodes.clone());
  nodes.first().unwrap().clone()
}

fn print(node: &Node,) {
  let output = node.unparse();
  println!("-> {}", output);
}
