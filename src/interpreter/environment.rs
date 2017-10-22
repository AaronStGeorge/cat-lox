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

    pub fn assign(&mut self, name: &Token, value: ExpressionReturn) -> Result<(), String> {
        match name {
            &Token::Ident(ref s) => if self.values.contains_key(s) {
                self.values.insert(s.clone(), Some(value));
                Ok(())
            } else {
                Err(String::from(
                    "Why the fuck would you try to assign \
                     to something that hasn't been defined?!?!",
                ))
            },
            _ => unreachable!(),
        }
    }

    pub fn define(&mut self, name: &Token, value: Option<ExpressionReturn>) -> () {
        match name {
            &Token::Ident(ref s) => {
                self.values.insert(s.clone(), value);
            }
            _ => unreachable!(),
        }
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
