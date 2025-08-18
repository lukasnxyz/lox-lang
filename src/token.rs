use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    LoxString,
    Number,

    // keywords
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

/*
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
        }
    }
}
*/

#[derive(Clone, Debug)]
#[derive(PartialOrd)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    None,
}

/// isEqual()
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Num(a), Object::Num(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::None, Object::None) => true,
            (Object::None, _) => false,
            _ => false,
        }
    }
}

impl Object {
    pub fn to_str(&self) -> Option<String> {
        match self {
            Object::Str(val) => Some(val.to_string()),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Object::Str(_) => true,
            _ => false,
        }
    }

    pub fn to_num(&self) -> Option<f64> {
        match self {
            Object::Num(val) => Some(*val),
            _ => None,
        }
    }

    pub fn is_num(&self) -> bool {
        match self {
            Object::Num(_) => true,
            _ => false,
        }
    }

    /// isTruthy() returns false for false and nil and true for everything else
    pub fn to_bool(&self) -> bool {
        match self {
            Object::Bool(val) => *val,
            Object::None => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} '{}' ", self.token_type, self.lexeme)?;
        match &self.literal {
            Object::Str(s) => write!(f, "{}", s),
            Object::Num(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::None => write!(f, "None"),
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Object, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            literal,
            line,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::Eof
    }
}
