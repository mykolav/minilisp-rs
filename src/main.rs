mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::{Interpreter, value::Value};

fn main() {
    let expressions = [
        "(+ 1 2)",
        "(do 9 8 7 6 5 4 3 2 1)",
        "(def x 42)",
        "(do (def x 42) x)",
        "(< 1 2)",
        "(> 1 2)",
        "(* 2 3)",
        "(if (< 1 2) 111 222)",
        "(if (> 1 2) 111 222)",
        "(do
            (def fact
                (lambda (x)
                    (if (< x 2) 
                        1 
                        (* x (fact (- x 1)))
                    )
                )
            )

            (fact 5)
        )"
    ];

    for expression in expressions {
        eval_and_print(expression)
    }
}

fn eval_and_print(expression: &str) -> () {
    let interpreter = Interpreter::new();
    
    let lexer = Lexer::new(expression);
    let mut parser = Parser::new(lexer);
    let program = parser.parse();

    let value = match interpreter.eval(&program) {
        Value::Number(number) => number.to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Lambda { 
            body: _, 
            params: _, 
            captured_env: _ } => String::from("#<lambda>"),
        Value::PredefinedFn(_) => String::from("#<predefined_fn>"),
        Value::Nil => String::from("nil"),
    };

    println!("{} evaluates to {}", expression, value)
}
