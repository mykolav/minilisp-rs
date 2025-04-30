use std::iter::Peekable;
use crate::lexer::{Lexer, Token};

#[derive(Clone, Debug)]
pub enum AtomOrList {
    Symbol(String),
    Number(i32),
    Bool(bool),
    List(Vec<AtomOrList>),
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer.peekable() }
    }

    pub fn parse(&mut self) -> AtomOrList {
        while let Some(token) = self.lexer.next() {
            // println!("Parser::parse::1: {:?}", token);
            return match token {
                Token::Symbol(symbol) => AtomOrList::Symbol(symbol),
                Token::Number(number) => AtomOrList::Number(number),
                Token::Bool(value) => AtomOrList::Bool(value),
                Token::LParen => {
                    let mut items = vec![];
                    while let Some(token) = self.lexer.peek() {
                        // println!("Parser::parse::2: {:?}", token);
                        if let Token::RParen = token {
                            self.lexer.next();
                            break
                        }

                        items.push(self.parse())
                    }

                    AtomOrList::List(items)
                },
                Token::RParen => panic!("Bad syntax!"),
            }
        }

        panic!("Unexpected end of input")
    }
}
