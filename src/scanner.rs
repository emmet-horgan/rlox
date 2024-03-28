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

    fn string(&mut self) {
        while (self.peek() != '"') && !self.at_end() {
            if self.peek() == '\n' {
                self._line += 1;
            }
            let _ = self.advance();
            let literal_slice = &self.source[self._start + 1..=self._current - 1];
            let source_slice = &self.source[self._start..=self._current];
            self.tokens.push(
                Token::new(
                    TokenType::String,
                    source_slice.iter().collect(),
                    Literal::String(literal_slice.iter().collect()),
                    self._line
                )
            );
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            let _ = self.advance();
        }
        let slice = &self.source[self._start ..self._current];
        let substring: String = slice.iter().collect();
        let id = self._keywords.get(&substring).cloned();
        if let Some(token_type) = id {
            self.tokens.push(
                Token::new(token_type, substring, Literal::None, self._line)
            );
        }
        else {
            self.tokens.push(
                Token::new(TokenType::Identifier, substring, Literal::None, self._line)
            );
        }
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            let _ = self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            let _ = self.advance();
            while self.peek().is_digit(10) {
                let _ = self.advance();
            }
        }
        let num_slice = &self.source[self._start..self._current];
        let num_string: String = num_slice.iter().collect();
        let num_f64 = num_string.parse::<f64>().unwrap();
        self.tokens.push(
            Token::new(
                TokenType::Number,
                num_string,
                Literal::Number(num_f64),
                self._line
            )
        );
    }

    fn match_char(&mut self, c: &char) -> bool {
        if self.at_end() {
            return false;
        }
        if &self.source[self._current] != c {
            return false;
        }

        self._current += 1;
        return true;
    }

    fn scan_token(&mut self) {
        if let Ok(c) = self.advance() {
            match c {
                '(' => self.tokens.push(Token::new(TokenType::LeftParen, "(".to_string(), Literal::None, self._line)),
                ')' => self.tokens.push(Token::new(TokenType::RightParen, ")".to_string(), Literal::None, self._line)),
                '{' => self.tokens.push(Token::new(TokenType::LeftBrace, "}".to_string(), Literal::None, self._line)),
                '}' => self.tokens.push(Token::new(TokenType::RightBrace, "{".to_string(), Literal::None, self._line)),
                ',' => self.tokens.push(Token::new(TokenType::Comma, ",".to_string(), Literal::None, self._line)),
                '.' => self.tokens.push(Token::new(TokenType::Dot, ".".to_string(), Literal::None, self._line)),
                '-' => self.tokens.push(Token::new(TokenType::Minus, "-".to_string(), Literal::None, self._line)),
                '+' => self.tokens.push(Token::new(TokenType::Plus, "+".to_string(), Literal::None, self._line)),
                ';' => self.tokens.push(Token::new(TokenType::SemiColon, ";".to_string(), Literal::None, self._line)),
                '*' => self.tokens.push(Token::new(TokenType::Star, "*".to_string(), Literal::None, self._line)),
                '!' => {
                    if self.match_char(&'=') {
                        self.tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), Literal::None, self._line));
                    }
                    else {
                        self.tokens.push(Token::new(TokenType::Bang, "!".to_string(), Literal::None, self._line))
                    }
                },
                '=' => {
                    if self.match_char(&'=') {
                        self.tokens.push(Token::new(TokenType::EqualEqual, "==".to_string(), Literal::None, self._line));
                    }
                    else {
                        self.tokens.push(Token::new(TokenType::Equal, "=".to_string(), Literal::None, self._line))
                    }
                },
                '<' => {
                    if self.match_char(&'=') {
                        self.tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), Literal::None, self._line));
                    }
                    else {
                        self.tokens.push(Token::new(TokenType::Less, "<".to_string(), Literal::None, self._line))
                    }
                },
                '>' => {
                    if self.match_char(&'=') {
                        self.tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), Literal::None, self._line));
                    }
                    else {
                        self.tokens.push(Token::new(TokenType::Greater, ">".to_string(), Literal::None, self._line))
                    }
                }
                '/' => {
                    if self.match_char(&'/') {
                        while self.peek() != '\n' && !self.at_end() {
                            let _ = self.advance();
                        }
                    }
                    else {
                        self.tokens.push(Token::new(TokenType::Slash, "/".to_string(), Literal::None, self._line))
                    }
                }
                ' ' | '\r' | '\t' => {},
                '\n' => {
                    self._line += 1
                },
                '"' => self.string(),
                _ => {
                    if c.is_digit(10) {
                        self.number();
                    } else if c.is_alphabetic(){
                        self.identifier()
                    } else {
                        println!("Error !");
                    }
                }
            }
        }
    }

    fn scan_tokens(&mut self) {
        while !self.at_end() {
            self._start = self._current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), Literal::None, self._line));
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
        scanner.scan_tokens();
        for x in scanner.tokens {
            println!("{}", x)
        }
        
    }

}