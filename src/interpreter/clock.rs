use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{Display, Formatter, Result};
use super::core::{Callable, CatBoxType, Interpreter};

#[derive(Debug)]
pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, __: Vec<CatBoxType>) -> CatBoxType {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        CatBoxType::Number(time as f64)
    }
}


impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "native clock function")
    }
}
