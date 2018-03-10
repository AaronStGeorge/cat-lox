use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use super::core::{Callable, Types, Interpreter};

pub struct Print {
    output_fn: Box<Fn(&str)>
}

impl Print {
    pub fn new(output_fn: Box<Fn(&str)>) -> Print {
        Print{output_fn: output_fn}
    }
}

impl Callable for Print {
    fn arity(&self) -> usize {
        1
    }

    fn call(&self, _: &mut Interpreter, params: Vec<Types>) -> Result<Types, String>  {
        (*self.output_fn)(&format!("{}", params[0]));
        Ok(Types::Nil)
    }
}

impl Debug for Print {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Print")
    }
}


impl Display for Print {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "native print function")
    }
}
