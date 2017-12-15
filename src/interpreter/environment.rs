use super::core::Types;
use super::clock::Clock;
use lexer::Token;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Environment {
    cactus_stack: Vec<Rc<RefCell<EnvironmentNode>>>,
}

impl Environment {
    pub fn global() -> Environment {
        Environment {
            cactus_stack: vec![Rc::new(RefCell::new(EnvironmentNode::global()))],
        }
    }

    pub fn new_from(environment: &Environment) -> Environment {
        Environment {
            cactus_stack: environment.cactus_stack.clone(),
        }
    }

    pub fn new_node(environment: &Environment) -> Environment {
        let mut new_environment = Environment {
            cactus_stack: environment.cactus_stack.clone(),
        };
        new_environment
            .cactus_stack
            .push(Rc::new(RefCell::new(EnvironmentNode::new())));

        new_environment
    }

    pub fn assign(&mut self, name: &Token, value: Types) -> Result<(), String> {
        match name {
            &Token::Ident(ref s) => {
                for e in self.cactus_stack.iter_mut().rev() {
                    if let Some(_) = e.borrow_mut().assign(s, &value) {
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

    pub fn define(&mut self, name: &Token, value: Option<Types>) -> () {
        match name {
            &Token::Ident(ref name) => {
                let len = self.cactus_stack.len();
                self.cactus_stack[len - 1].borrow_mut().define(name, value)
            }
            _ => unreachable!(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Option<Types>, String> {
        match name {
            &Token::Ident(ref name) => {
                for e in self.cactus_stack.iter().rev() {
                    if let Some(value) = e.borrow().get(name) {
                        return Ok(value);
                    }
                }
                Err(format!("{} is super fucking undefined", name))
            }
            _ => unreachable!(),
        }
    }
}


#[derive(Debug)]
struct EnvironmentNode {
    values: HashMap<String, Option<Types>>,
}

impl EnvironmentNode {
    fn new() -> EnvironmentNode {
        EnvironmentNode {
            values: HashMap::new(),
        }
    }

    // The global environment, all native functions should be defined here.
    fn global() -> EnvironmentNode {
        let mut global = EnvironmentNode {
            values: HashMap::new(),
        };

        let clock = Types::Callable(Rc::new(Box::new(Clock {})));

        global.define("clock", Some(clock));

        global
    }

    fn assign(&mut self, name: &str, value: &Types) -> Option<()> {
        if self.values.contains_key(name) {
            self.values.insert(String::from(name), Some(value.clone()));
            Some(())
        } else {
            None
        }
    }

    fn define(&mut self, name: &str, value: Option<Types>) -> () {
        self.values.insert(String::from(name), value);
    }

    fn get(&self, name: &str) -> Option<Option<Types>> {
        match self.values.get(name) {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
}
