use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Symbol(String),
    Number(i32),
    Bool(bool)
}

pub struct Lexer<'a> {
    expr_chars: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(expression: &'a str) -> Lexer<'a> {
        Lexer { expr_chars: expression.chars().peekable() }
    }

    fn is_separator(ch: char) -> bool {
        ch.is_whitespace() || ch == '(' || ch == ')'
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let ch = self.expr_chars.find(|&ch| !ch.is_whitespace())?;

        // println!("Lexer::next: {}", ch);

        if ch == '(' {
            return Some(Token::LParen)
        }

        if ch == ')' {
            return Some(Token::RParen)
        }

        let mut symbol = String::from(ch);
        while let Some(&ch) = self.expr_chars.peek() {
            if Self::is_separator(ch) {
                break;
            }

            self.expr_chars.next();
            symbol.push(ch);
        }

        if symbol == "true" {
            return Some(Token::Bool(true))
        }

        if symbol == "false" {
            return Some(Token::Bool(false))
        }

        match symbol.parse::<i32>() {
            Ok(number) => Some(Token::Number(number)),
            Err(_) => Some(Token::Symbol(symbol))
        }
    }
}
