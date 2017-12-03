use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{Display, Formatter, Result as FmtResult};
use super::core::{Callable, Types, Interpreter};

#[derive(Debug)]
pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, __: Vec<Types>) -> Result<Types, String>  {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(Types::Number(time as f64))
    }
}


impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "native clock function")
    }
}
