mod error {
  pub type Error = String;
}

mod primitives {
  #[derive(Debug, Clone)]
  pub enum Primitive {
    Undefined,
    Null,
    Boolean(bool),
    Number(f32),
    String(String),
    Array(Vec<Primitive>),
  }
}

mod scopes {

  use crate::primitives::Primitive;
  use std::collections::HashMap;
  pub type Scope = HashMap<String, Bound>;

  pub type Scopes = Vec<Scope>;

  pub enum Bound {
    Let(Primitive),
    Const(Primitive),
  }
}

mod vm {

  mod instructions {
    pub use crate::error::Error;
    pub use crate::scopes::Scopes;
    pub use executable::Executable;
    pub use fork::Fork;
    pub use get::Get;
    pub use insert_set::InsertSet;
    pub use jump::Jump;
    pub use push::Push;

    pub use set::Set;

    mod executable {
      use crate::{error::Error, scopes::Scopes, vm::Stack};
      pub trait Executable {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error>;
      }
    }

    mod get {
      use crate::{
        error::Error,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };

      /// Gets variable from scopes by following the parent scopes all the
      /// way up and pushes the value on the stack.  If the variable is
      /// not defined then it pushes a `Primitive::Undefined` to the stack
      pub struct Get(String);

      impl Get {
        pub fn new(variable: &str) -> Self {
          unimplemented!();
        }
      }

      impl Executable for Get {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error> {
          unimplemented!();
        }
      }
    }

    mod insert_set {
      use crate::{
        error::Error,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };
      pub enum Bind {
        Let,
        Const,
      }

      pub struct InsertSet(String, Bind);

      /// Pops the last value from the stack and stores it in the current
      /// scope with the given name and binding type.  If the value is
      /// already in the given scope then it returns an error
      impl InsertSet {
        pub fn new(variable: &str) -> Self {
          unimplemented!()
        }
      }

      impl Executable for InsertSet {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error> {
          unimplemented!();
        }
      }
    }

    mod set {
      use crate::{
        error::Error,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };

      pub struct Set(String);

      /// Pops the last value from the stack and searches for an existing
      /// binding with a given name in the scopes by following the parent
      /// scopes all the way up and overrides its value with the popped
      /// one.  If it couldn't find any bindings it returns an error.  If
      /// the binding found is a `Bound::Const` then it returns an error
      impl Set {
        pub fn new(variable: &str) -> Self {
          unimplemented!()
        }
      }

      impl Executable for Set {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error> {
          unimplemented!();
        }
      }
    }

    mod jump {
      use crate::{
        error::Error,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };

      pub struct Jump(usize);

      /// Jumps to the given instruction at index
      impl Jump {
        fn new(index: usize) -> Self {
          unimplemented!()
        }
      }

      impl Executable for Jump {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error> {
          unimplemented!();
        }
      }
    }

    mod fork {
      use crate::{
        error::Error,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };

      /// Pops the last value from the stack and checks its boolean value
      /// and if it is `true` then it jumps to the first index otherwise
      /// it jumps to the second index
      pub struct Fork(usize, usize);

      impl Fork {
        pub fn new(true_index: usize, false_index: usize) -> Self {
          unimplemented!()
        }
      }
    }

    mod push {
      use crate::{
        error::Error,
        primitives::Primitive,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };

      // Pushes a `Primitive` value to the stack
      pub struct Push(Primitive);

      impl Push {
        pub fn new(value: Primitive) -> Self {
          unimplemented!();
        }
      }

      impl Executable for Push {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error> {
          unimplemented!();
        }
      }
    }

    mod pop {
      use crate::{
        error::Error,
        scopes::Scopes,
        vm::{instructions::executable::Executable, Stack},
      };

      /// Pops the last value from the stack
      pub struct Pop;

      impl Pop {
        pub fn new() -> Self {
          unimplemented!();
        }
      }

      impl Executable for Pop {
        fn execute(
          &self,
          index: usize,
          locals: &mut Scopes,
          stack: &mut Stack,
        ) -> Result<Option<usize>, Error> {
          unimplemented!();
        }
      }
    }

  }

  use crate::{
    error::Error,

    primitives::Primitive,
    scopes::Scopes,
    vm::instructions::{Executable, Fork, Get, InsertSet, Jump, Pop, Push, Set},
  };


  pub enum Instruction {
    InsertSet(InsertSet),
    Set(Set),
    Get(Get),
    Jump(Jump),
    Fork(Fork),
    Push(Push),
    Pop(Pop),
    //   Open(Open),
    //   Close(Close),
    //   Exit(Exit),
  }
  impl Executable for Instruction {
    fn execute(
      &self,
      index: usize,
      locals: &mut Scopes,
      stack: &mut Stack,
    ) -> Result<Option<usize>, Error> {
      unimplemented!();
    }
  }

  pub type Stack = Vec<Primitive>;
  pub struct VirtualMachine {
    scopes: Scopes,
    stack: Stack,
  }
  impl VirtualMachine {
    fn execute(&mut self, instructions: &[Instruction]) -> Result<(), Error> {
      let mut i = 0;
      loop {
        match instructions[i].execute(i, &mut self.scopes, &mut self.stack) {
          Ok(Some(next)) => i = next,
          Ok(None) => return Ok(()),
          _ => return Err("Oups, something went terribly wrong".to_string()),
        }
      }
    }
  }
}


// pub enum Declare {
//   Let(String, Option<Expression>),
//   Const(String, Expression),
// }

// impl Declare {
//   fn eval(self, scope: &mut Scope) -> Primitive {
//     match self {
//       Declare::Let(name, value) => {
//         let primitive = match value {
//           Some(expression) => expression.eval(scope),
//           None => Primitive::Undefined,
//         };
//         scope.insert(name, primitive);
//         Primitive::Undefined
//       }
//       _ => Primitive::Undefined,
//     }
//   }
// }

// // TODO add parent scope lookup
// pub struct Lookup(pub String);

// impl Lookup {
//   fn eval(self, scope: &mut Scope) -> Primitive {
//     match scope.get(&self.0) {
//       Some(p) => p.clone(),
//       None => Primitive::Undefined,
//     }
//   }
// }

// pub enum Instruction {
//   Declare(Declare),
//   Lookup(Lookup),
// }

// impl Instruction {
//   fn eval(self, scope: &mut Scope) -> Primitive {
//     match self {
//       Instruction::Declare(d) => d.eval(scope),
//       Instruction::Lookup(l) => l.eval(scope),
//     }
//   }
// }

// pub enum Expression {
//   Primitive(Primitive),
//   Instruction(Box<Instruction>),
// }

// impl Expression {
//   fn eval(self, scope: &mut Scope) -> Primitive {
//     match self {
//       Expression::Primitive(p) => p,
//       Expression::Instruction(i) => i.eval(scope),
//     }
//   }
// }

// /// let x = 1
// /// let y = x
// pub fn eval(
//   global: &mut Scope,
//   _locals: &mut Scopes,
//   expressions: &mut Vec<Expression>,
// ) -> Result<(), String> {
//   // Err("Error".to_string())
//   for expression in expressions.drain(..) {
//     expression.eval(global);
//   }
//   Ok(())
// }
