use {
  crate::{
    lexer::lex_all,
    node::{atom::Atom, function::Function, list::List, Node},
    parser::{parse_all, Unparse},
  },
  std::collections::HashMap,
};

pub struct Env {
  map: HashMap<String, Node>,
}

impl Env {
  #[must_use]
  pub fn new() -> Self {
    let mut env = Self {
      map: HashMap::new(),
    };

    env.define("+".to_string(), Node::Function(Box::new(Function::add())));
    env.define("-".to_string(), Node::Function(Box::new(Function::sub())));
    env
  }

  #[must_use]
  pub fn get(&self, name: &str) -> Option<&Node> {
    self.map.get(name)
  }

  pub fn define(&mut self, name: String, val: Node) {
    self.map.insert(name, val);
  }

  pub fn parse_and_eval(&mut self, input: &str) -> Node {
    let tokens = lex_all(input);
    parse_all(&tokens)
      .first()
      .map_or_else(|| Node::Unit, |node| self.eval(node))
  }

  pub fn parse_eval_unparse(&mut self, input: &str) -> String {
    self.parse_and_eval(input).unparse()
  }

  pub fn eval(&mut self, node: &Node) -> Node {
    match node {
      Node::Unit => Node::Unit,
      Node::Atom(atom) => match atom {
        Atom::Symbol(_) | Atom::Number(_) => node.clone(),
      },
      Node::List(list) => match &**list {
        List::Single(first) => {
          match &**first {
            Node::Unit => Node::Unit,
            Node::Atom(_) => *first.clone(),
            Node::List(_) => todo!(),
            Node::Function(_) => todo!(),
        }
        },
        List::Multi { first, rest } => match &**first {
          Node::Unit => Node::Unit,
          Node::Atom(atom) => match atom {
            Atom::Number(_) => todo!(),
            Atom::Symbol(sym) => match self.get(sym) {
              Some(Node::Function(func)) => {
                let args = &rest;
                func.0(args)
              },
              Some(_) => todo!(),
              None => todo!(),
            },
          },
          Node::List(_) => todo!(),
          Node::Function(_) => todo!(),
        },
      },
      Node::Function(_) => todo!(),
    }
  }
}

impl Default for Env {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
#[path = "./tests/eval_tests.rs"]
mod eval_tests;
