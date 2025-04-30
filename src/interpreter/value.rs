use std::rc::Rc;
use crate::parser::AtomOrList;
use crate::interpreter::env::EnvFrame;

#[derive(Clone, Debug)]
pub enum Value {
    Number(i32),
    Bool(bool),
    Lambda { body: AtomOrList, params: Vec<String>, captured_env: Rc<EnvFrame> },
    PredefinedFn(PredefinedFnKind),
    Nil,
}

#[derive(Clone, Debug)]
pub enum PredefinedFnKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Gt,
    Lt,
}

impl PredefinedFnKind {
    pub fn apply(&self, args: &[Value]) -> Value {
        if args.len() != 2 { panic!("Expected two arguments") }
        let Value::Number(arg0) = args[0] else { panic!("The first argument must be a Value::Number") };
        let Value::Number(arg1) = args[1] else { panic!("The second argument must be a Value::Number") };

        match self {
            PredefinedFnKind::Add => Value::Number(arg0 + arg1),
            PredefinedFnKind::Sub => Value::Number(arg0 - arg1),
            PredefinedFnKind::Mul => Value::Number(arg0 * arg1),
            PredefinedFnKind::Div => Value::Number(arg0 / arg1),
            PredefinedFnKind::Eq => Value::Bool(arg0 == arg1),
            PredefinedFnKind::Gt => Value::Bool(arg0 > arg1),
            PredefinedFnKind::Lt => Value::Bool(arg0 < arg1),
        }
    }
}
