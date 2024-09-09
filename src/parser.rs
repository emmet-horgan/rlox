use crate::token::{self, TokenType, Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(),
    Get(),
    Grouping(Grouping),
    Literal(Literal),
    Logical(),
    Set(),
    Super(),
    This(),
    Unary(Unary),
    Variable(),
}

#[derive(Clone, Debug)]
pub struct Assign {
    name: String,
    expr: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Binary {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Grouping {
    expr: Box<Expr>
}

#[derive(Clone, Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    False,
    True,
    Nil
}

#[derive(Clone, Debug)]
pub struct Unary {
    op: Token,
    right: Box<Expr>
}

#[derive(Clone, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    curr: usize
}

impl Parser {

    fn consume(&mut self, token_type: &TokenType, message: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        panic!("{}", message);
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        return &self.peek().token_type == token_type;

    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.curr += 1;
        }
        self.previous()
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.curr].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.curr-1].clone()
    }

    pub fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary { left: Box::new(expr), op: op, right: Box::new(right) })
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        
        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Binary { left: Box::new(expr), op: op, right: Box::new(right) })
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Binary { left: Box::new(expr), op: op, right: Box::new(right) })
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary { left: Box::new(expr), op: op, right: Box::new(right) })
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary { op: op, right: Box::new(right) });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::False]) {
            return Expr::Literal(Literal::False);
        }
        if self.match_tokens(&[TokenType::True]) {
            return Expr::Literal(Literal::True);
        }
        if self.match_tokens(&[TokenType::Nil]) {
            return Expr::Literal(Literal::Nil);
        }
        if self.match_tokens(&[TokenType::Number]) {
            return Expr::Literal(Literal::Number(match self.previous().literal {
                token::Literal::Number(x) => x,
                _ => 0.0
            }));
        }
        if self.match_tokens(&[TokenType::String]) {
            return Expr::Literal(Literal::String(match self.previous().literal {
                token::Literal::String(x) => x,
                _ => String::new()
            }));
        }
        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expr();
            self.consume(&TokenType::RightParen, "Expect ')' after expression".to_owned());
            return Expr::Grouping(Grouping { expr: Box::new(expr) });
        }
        println!("[DEBUG] {:?}", self);
        panic!("Expect expression");

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scanner;

    #[test]
    fn test_parser() {
        let expr = "43 + (76 + 56 / (4 * 4))";
        let mut stream = Scanner::from(expr);
        stream.scan_tokens();
        let mut parser = Parser {tokens: stream.tokens.clone(), curr: 0};
        let parsed_expr = parser.expr();
        println!("{:?}", parsed_expr);
    }
}