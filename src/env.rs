use {
  crate::node::{atom::Atom, Node},
  std::collections::HashMap,
};

pub type LispFunction = Box<dyn Fn(&[Node],) -> Node,>;
pub struct Function(pub LispFunction,);

pub struct Env {
  map: HashMap<String, Function,>,
}

impl Env {
  #[must_use]
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }

  #[must_use]
  pub fn get(&self, name: &str,) -> Option<&Function,> {
    self.map.get(name,)
  }

  pub fn define(&mut self, name: String, val: Function,) {
    self.map.insert(name, val,);
  }

  pub fn eval(&mut self, node: &Node,) -> Node {
    match node {
      Node::Unit => Node::Unit,
      Node::Atom(atom,) => match atom {
        Atom::Number(_,) => node.clone(),
        Atom::Symbol(_,) => todo!(),
      },
      Node::List(list,) => {
        if let Some(Node::Atom(Atom::Symbol(sym,),),) = list.0.first() {
          if let Some(Function(func,),) = self.get(sym,) {
            let args = &list.0[1..];
            func(args,)
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
