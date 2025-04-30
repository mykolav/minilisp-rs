use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::interpreter::value::Value;

#[derive(Clone, Debug)]
pub struct EnvFrame {
    bindings: HashMap<String, Value>,
    outer: Option<Rc<RefCell<EnvFrame>>>
}

impl EnvFrame {
    pub fn new(outer: Option<&Rc<RefCell<EnvFrame>>>) -> EnvFrame {
        EnvFrame {
            bindings: HashMap::new(),
            outer: outer.map(|it| it.clone()),
        }
    }

    pub fn bind(&mut self, symbol: String, value: Value) -> &mut EnvFrame {
        self.bindings.insert(symbol, value);
        self
    }

    pub fn resolve(&self, symbol: &str) -> Option<Value> {
        if let Some(value) = self.bindings.get(symbol) {
            return Some(value.clone())
        }

        if let Some(outer_env_frame) = self.outer.as_ref() {
            return outer_env_frame.borrow().resolve(symbol)
        }

        None
    }
}
