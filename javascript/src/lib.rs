use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Primitive {
  Undefined,
  Null,
  Boolean(bool),
  Number(f32),
  String(String),
  Array(Vec<Primitive>),
}

pub enum Declare {
  Let(String, Option<Expression>),
  Const(String, Expression),
}

impl Declare {
  fn eval(self, scope: &mut Scope) -> Primitive {
    match self {
      Declare::Let(name, value) => {
        let primitive = match value {
          Some(expression) => expression.eval(scope),
          None => Primitive::Undefined,
        };
        scope.insert(name, primitive);
        Primitive::Undefined
      }
      _ => Primitive::Undefined,
    }
  }
}

// TODO add parent scope lookup
pub struct Lookup(pub String);

impl Lookup {
  fn eval(self, scope: &mut Scope) -> Primitive {
    match scope.get(&self.0) {
      Some(p) => p.clone(),
      None => Primitive::Undefined,
    }
  }
}

pub enum Instruction {
  Declare(Declare),
  Lookup(Lookup),
}

impl Instruction {
  fn eval(self, scope: &mut Scope) -> Primitive {
    match self {
      Instruction::Declare(d) => d.eval(scope),
      Instruction::Lookup(l) => l.eval(scope),
    }
  }
}

pub enum Expression {
  Primitive(Primitive),
  Instruction(Box<Instruction>),
}

impl Expression {
  fn eval(self, scope: &mut Scope) -> Primitive {
    match self {
      Expression::Primitive(p) => p,
      Expression::Instruction(i) => i.eval(scope),
    }
  }
}

pub type Scope = HashMap<String, Primitive>;

pub type Scopes = Vec<Scope>;

/// let x = 1
/// let y = x
pub fn eval(
  global: &mut Scope,
  _locals: &mut Scopes,
  expressions: &mut Vec<Expression>,
) -> Result<(), String> {
  // Err("Error".to_string())
  for expression in expressions.drain(..) {
    expression.eval(global);
  }
  Ok(())
}
