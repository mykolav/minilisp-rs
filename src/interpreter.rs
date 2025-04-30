use std::rc::Rc;
use crate::parser::AtomOrList;
use crate::interpreter::env::EnvFrame;
use crate::interpreter::value::{Value, PredefinedFnKind};

pub mod value;
mod env;

pub struct Interpreter {
    env: Rc<EnvFrame>
}

impl Interpreter {
    pub fn new() -> Interpreter {

        let root_env_frame = Rc::new(EnvFrame::new(None));

        root_env_frame
            .bind(String::from("+"), Value::PredefinedFn(PredefinedFnKind::Add))
            .bind(String::from("-"), Value::PredefinedFn(PredefinedFnKind::Sub))
            .bind(String::from("*"), Value::PredefinedFn(PredefinedFnKind::Mul))
            .bind(String::from("/"), Value::PredefinedFn(PredefinedFnKind::Div))
            .bind(String::from("<"), Value::PredefinedFn(PredefinedFnKind::Lt))
            .bind(String::from(">"), Value::PredefinedFn(PredefinedFnKind::Gt))
            .bind(String::from("="), Value::PredefinedFn(PredefinedFnKind::Eq));

        Interpreter {
            env: root_env_frame
        }
    }

    pub fn eval(&self, program: &AtomOrList) -> Value {
        self.do_eval(program, &self.env)
    }

    fn do_eval(&self, program: &AtomOrList, env: &Rc<EnvFrame>) -> Value {
        match program {
            AtomOrList::Symbol(symbol) => 
                env.resolve(symbol)
                   .expect(&format!("Failed to resolve symbol '{}'", symbol))
                   .clone(),

            AtomOrList::Number(number) => Value::Number(*number),
            AtomOrList::Bool(value) => Value::Bool(*value),
            
            AtomOrList::List(items) => {
                if items.is_empty() {
                    return Value::Nil
                }

                if let AtomOrList::Symbol(symbol) = &items[0] {
                    if symbol == "def" {
                        let AtomOrList::Symbol(id) = &items[1] 
                            else { panic!("def requires a symbol as the first argument") };

                        let value = self.do_eval(&items[2], env);

                        env.bind(id.clone(), value);

                        return Value::Nil
                    }

                    if symbol == "do" {
                        let mut value = Value::Nil;
                        for i in 1..items.len() {
                            value = self.do_eval(&items[i], env)
                        }

                        return value
                    }

                    if symbol == "if" {
                        let cond = self.do_eval(&items[1], env);
                        return if Self::is_truthy(&cond) {
                            self.do_eval(&items[2], env)
                        } else {
                            self.do_eval(&items[3], env)
                        }
                    }

                    if symbol == "lambda" {
                        let AtomOrList::List(params) = items[1].clone()
                            else { panic!("lambda requires a list of parameters") };

                        let param_names = params
                            .into_iter()
                            .map(|it| { 
                                let AtomOrList::Symbol(symbol) = it 
                                    else { panic!("lambda parameters must be symbols") }; 
                                symbol
                            })
                            .collect();

                        return Value::Lambda { 
                            body: items[2].clone(), 
                            params: param_names, 
                            captured_env: env.clone() }; 
                    }
                }

                let values: Vec<Value> = items.iter()
                    .map(|it| self.do_eval(it, env))
                    .collect();

                // `values` are not empty since the execution folow only reaches this 
                // line if `items` are not empty
                let lambda_or_fn = values.first().unwrap();
                let args = &values[1..];

                if let Value::PredefinedFn(predefined_fn) = lambda_or_fn {
                    return predefined_fn.apply(args)
                }

                if let Value::Lambda { 
                    body, 
                    params, 
                    captured_env } = lambda_or_fn {
                        return self.do_eval(
                            body, 
                            &Rc::new(
                                Self::build_lambda_env_frame(captured_env, params, args))
                        )
                }

                panic!("Cannot apply non-function {:#?}", lambda_or_fn);
            },
        }
    }

    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Nil         => false,
            Value::Bool(false) => false,
            _                  => true,
        }
    }

    fn build_lambda_env_frame(
        captured_env: &Rc<EnvFrame>,  params: &[String], args: &[Value]
    ) -> EnvFrame {
        let env_frame = EnvFrame::new(Some(captured_env));
        for i in 0..params.len() {
            env_frame.bind(params[i].clone(), args[i].clone());
        }

        env_frame
    }
}
