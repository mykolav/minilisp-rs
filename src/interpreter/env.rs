use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::interpreter::value::Value;

#[derive(Clone, Debug)]
pub struct EnvFrame {
    bindings: RefCell<HashMap<String, Value>>,
    outer: Option<Rc<EnvFrame>>
}

impl EnvFrame {
    pub fn new(outer: Option<&Rc<EnvFrame>>) -> EnvFrame {
        EnvFrame {
            bindings: RefCell::new(HashMap::new()),
            outer: outer.map(|it| it.clone()),
        }
    }

    pub fn bind(&self, symbol: String, value: Value) -> &EnvFrame {
        self.bindings.borrow_mut().insert(symbol, value);
        self
    }

    pub fn resolve(&self, symbol: &str) -> Option<Value> {
        if let Some(value) = self.bindings.borrow().get(symbol) {
            return Some(value.clone())
        }

        if let Some(outer_env_frame) = self.outer.as_ref() {
            return outer_env_frame.resolve(symbol)
        }

        None
    }
}
