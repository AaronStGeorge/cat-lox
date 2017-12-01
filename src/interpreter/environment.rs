use super::core::CatBoxType;
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

    pub fn new_from(environment : &Environment) -> Environment {
        Environment {
            cactus_stack: environment.cactus_stack.clone(),
        }
    }

    // TODO: name this better
    pub fn open(&mut self) {
        self.cactus_stack.push(Rc::new(RefCell::new(EnvironmentNode::new())));
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

    pub fn assign(&mut self, name: &Token, value: CatBoxType) -> Result<(), String> {
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

    pub fn define(&mut self, name: &Token, value: Option<CatBoxType>) -> () {
        match name {
            &Token::Ident(ref name) => {
                let len = self.cactus_stack.len();
                self.cactus_stack[len - 1].borrow_mut().define(name, value)
            }
            _ => unreachable!(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Option<CatBoxType>, String> {
        match name {
            &Token::Ident(ref name) => {
                for e in self.cactus_stack.iter().rev() {
                    if let Some(value) = e.borrow().get(name) {
                        return Ok(value);
                    }
                }
                Err(String::from("That variable is super fucking undefined"))
            }
            _ => unreachable!(),
        }
    }
}


#[derive(Debug)]
struct EnvironmentNode {
    values: HashMap<String, Option<CatBoxType>>,
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

        let clock = CatBoxType::Callable(Rc::new(Box::new(Clock {})));

        global.define("clock", Some(clock));

        global
    }

    fn assign(&mut self, name: &str, value: &CatBoxType) -> Option<()> {
        if self.values.contains_key(name) {
            self.values.insert(String::from(name), Some(value.clone()));
            Some(())
        } else {
            None
        }
    }

    fn define(&mut self, name: &str, value: Option<CatBoxType>) -> () {
        self.values.insert(String::from(name), value);
    }

    fn get(&self, name: &str) -> Option<Option<CatBoxType>> {
        match self.values.get(name) {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }
}
