use crate::token::{TokenType, Token};
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

pub struct Assign {
    name: String,
    expr: Box<Expr>
}

pub struct Binary {
    left: Box<Expr>,
    op: Token,
    right: Box<Expr>
}

pub struct Grouping {
    expr: Box<Expr>
}

pub enum Literal {
    Number(f64),
    String(String),
    False,
    True,
    Nil
}

pub struct Unary {
    op: Token,
    right: Box<Expr>
}


pub struct Parser {
    tokens: Vec<Token>,
    curr: usize
}

impl Parser {

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
                
            }))
        }
    }
}