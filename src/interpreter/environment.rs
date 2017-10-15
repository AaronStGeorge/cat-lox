use std::collections::HashMap;
use super::core::ExpressionReturn;
use lexer::Token;

pub struct Environment {
    values: HashMap<String, Option<ExpressionReturn>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<ExpressionReturn>) -> () {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Option<ExpressionReturn>, String> {
        match name {
            &Token::Ident(ref s) => match self.values.get(s) {
                Some(e) => Ok(e.clone()),
                None => Err(String::from("Fucking fuck, that variable is not defined")),
            },
            _ => unreachable!(),
        }
    }
}
