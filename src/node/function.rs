use {
  super::{atom::Atom, Node},
  std::{fmt, rc::Rc},
};

pub type Alias = Rc<dyn Fn(&[Node]) -> Node>;

#[derive(Clone)]
pub struct Function(pub Alias);

impl PartialEq for Function {
  fn eq(&self, _other: &Self) -> bool {
    // Function comparison is inherently not supported.
    // Compare based on identity or other metadata if needed.
    false
  }
}

impl fmt::Debug for Function {
  fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
    write!(format, "Function")
  }
}

impl Function {
  #[must_use]
  pub fn add() -> Self {
    Self(Rc::new(|args: &[Node]| -> Node {
      let mut sum = 0;
      for arg in args {
        if let Node::Atom(Atom::Number(n)) = arg {
          sum += n;
        }
      }
      Node::Atom(Atom::Number(sum))
    }))
  }

  #[must_use]
  pub fn mul() -> Self {
    Self(Rc::new(|args: &[Node]| -> Node {
      let mut result = 1;
      for arg in args {
        if let Node::Atom(Atom::Number(n)) = arg {
          result *= n;
        }
      }
      Node::Atom(Atom::Number(result))
    }))
  }

  #[must_use]
  pub fn sub() -> Self {
    Self(Rc::new(|args: &[Node]| -> Node {
      let result: i64 = match args {
        [Node::Atom(Atom::Number(left))] => 0 - left,
        [Node::Atom(Atom::Number(left)), Node::Atom(Atom::Number(right))] => left - right,
        _ => todo!(),
      };
      Node::Atom(Atom::Number(result))
    }))
  }

  #[must_use]
  pub fn div() -> Self {
    Self(Rc::new(|args: &[Node]| -> Node {
      let result: i64 = match args {
        [Node::Atom(Atom::Number(left))] => 1 / left,
        [Node::Atom(Atom::Number(left)), Node::Atom(Atom::Number(right))] => left / right,
        _ => todo!(),
      };
      Node::Atom(Atom::Number(result))
    }))
  }
}
