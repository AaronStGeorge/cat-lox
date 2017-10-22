use std::collections::HashMap;
use super::core::ExpressionReturn;
use lexer::Token;

pub struct Environment {
    cactus_stack: Vec<EnvironmentNode>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            cactus_stack: vec![EnvironmentNode::new()],
        }
    }

    // TODO: name this better
    pub fn open(&mut self) {
        self.cactus_stack.push(EnvironmentNode::new());
    }

    // TODO: name this better
    pub fn close(&mut self) -> Result<(), String> {
        if self.cactus_stack.len() <= 1 {
            return Err(String::from(
                "You can't close the fucking global environment!",
            ));
        }
        self.cactus_stack.pop();
        Ok(())
    }

    pub fn assign(&mut self, name: &Token, value: ExpressionReturn) -> Result<(), String> {
        match name {
            &Token::Ident(ref s) => {
                for e in self.cactus_stack.iter_mut().rev() {
                    if let Some(_) = e.assign(s, &value) {
                        return Ok(());
                    }
                }
                Err(String::from(
                    "Why the fuck would you try to assign \
                     to something that hasn't been defined?!?!",
                ))
            }
            _ => unreachable!(),
        }
    }

    pub fn define(&mut self, name: &Token, value: Option<ExpressionReturn>) -> () {
        match name {
            &Token::Ident(ref name) => {
                let len = self.cactus_stack.len();
                self.cactus_stack[len - 1].define(name, value)
            }
            _ => unreachable!(),
        }
    }

    pub fn get(&mut self, name: &Token) -> Result<Option<ExpressionReturn>, String> {
        match name {
            &Token::Ident(ref name) => {
                for e in self.cactus_stack.iter_mut().rev() {
                    if let Some(value) = e.get(name) {
                        return Ok(value);
                    }
                }
                Err(String::from("That variable is super fucking undefined"))
            }
            _ => unreachable!(),
        }
    }
}

struct EnvironmentNode {
    values: HashMap<String, Option<ExpressionReturn>>,
}

impl EnvironmentNode {
    fn new() -> EnvironmentNode {
        EnvironmentNode {
            values: HashMap::new(),
        }
    }

    fn assign(&mut self, name: &str, value: &ExpressionReturn) -> Option<()> {
        if self.values.contains_key(name) {
            self.values.insert(String::from(name), Some(value.clone()));
            Some(())
        } else {
            None
        }
    }

    fn define(&mut self, name: &str, value: Option<ExpressionReturn>) -> () {
        self.values.insert(String::from(name), value);
    }

    fn get(&self, name: &str) -> Option<Option<ExpressionReturn>> {
        match self.values.get(name) {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
}
