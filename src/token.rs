#[derive(Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::And => write!(f, "And"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Class => write!(f, "Class"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Else => write!(f, "Else"),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::False => write!(f, "False"),
            TokenType::For => write!(f, "For"),
            TokenType::Fun => write!(f, "Fun"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::If => write!(f, "If"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Nil => write!(f, "Nil"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Return => write!(f, "Return"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::SemiColon => write!(f, "SemiColon"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::String => write!(f, "String"),
            TokenType::Super => write!(f, "Super"),
            TokenType::This => write!(f, "This"),
            TokenType::True => write!(f, "True"),
            TokenType::Var => write!(f, "Var"),
            TokenType::While => write!(f, "While"),
            _ => write!(f, "Uknown")
        }
        
    }
}

pub enum Literal {
    Number(f64),
    String(String),
    None
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(x) => write!(f, "Number({})", x),
            Literal::String(x) => write!(f, "String({})", x),
            Literal::None => write!(f, "None")
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token { 
            token_type: token_type, 
            lexeme: lexeme, 
            literal: literal, 
            line: line 
        }
    }
}

impl std::fmt::Display for Token {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] => {} as {}", self.token_type, self.lexeme, self.literal)
    }
}