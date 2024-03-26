use std::default::Default;
use crate::token::{Token, TokenType, Literal};
use std::collections::HashMap;

#[derive(Default)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    _keywords: HashMap<String, TokenType>,
    _start: usize,
    _current: usize,
    _line: usize
}

impl Scanner {

    pub fn new() -> Self {
        Scanner {
            source: Default::default(),
            tokens: Vec::new(),
            _keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
            _start: 0,
            _current: 0,
            _line: 1
        }
    }

    pub fn from(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            _keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ]),
            _start: 0,
            _current: 0,
            _line: 1
        }
    }

    fn advance(&mut self) -> Result<char, ()> {
        if self._current >= self.source.len() {
            return Err(());
        }
        let cur_char = self.source[self._current];
        self._current += 1;
        Ok(cur_char)
    }

    fn at_end(&self) -> bool {
        self._current >= self.source.len()
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_alphanumeric()
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        } else {
            return self.source[self._current];
        }
    }

    fn peek_next(&self) -> char {
        if self._current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source[self._current + 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let program: &str = 
            "var variable = 5;
            class Class {
                this.variable = 4
            }";
        let mut scanner = Scanner::from(program);
        while let Ok(c) = scanner.advance() {
            print!("{}", c);
        }
    }

}