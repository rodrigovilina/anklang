use {
  crate::{atom::Atom, node::Node},
  std::collections::HashMap,
};

pub type LispFunction = Box<dyn Fn(&[Node],) -> Node,>;
pub struct Function(pub LispFunction,);

pub struct Env {
  map: HashMap<String, Function,>,
}

impl Env {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }

  pub fn get(&self, name: &str,) -> Option<&Function,> {
    self.map.get(name,)
  }

  pub fn define(&mut self, name: String, val: Function,) {
    self.map.insert(name, val,);
  }

  pub fn eval(&mut self, node: &Node,) -> Node {
    match node {
      Node::Unit => Node::Unit,
      Node::Atom(a,) => match a {
        Atom::Number(_,) => node.clone(),
        Atom::Symbol(_,) => todo!(),
      },
      Node::List(l,) => {
        if let Some(Node::Atom(Atom::Symbol(s,),),) = l.0.first() {
          if let Some(Function(f,),) = self.get(s,) {
            let args = &l.0[1..];
            f(args,)
          } else {
            node.clone()
          }
        } else {
          todo!("First list element is not a symbol")
        }
      },
    }
  }
}

impl Default for Env {
  fn default() -> Self {
    Self::new()
  }
}
