use std::{
    collections::HashMap,
    env, fmt, fs,
    io::{self, Write},
};

#[derive(Debug)]
pub enum LoxError<'a> {
    Io(io::Error),
    ParseFloatError(std::num::ParseFloatError),
    InvalidPrompt(String),
    EndOfFile,
    CodeError(usize, &'a str, &'a str), // line, where, msg
}

impl<'a> fmt::Display for LoxError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Io(e) => write!(f, "io error: {}", e),
            LoxError::ParseFloatError(e) => write!(f, "parse float error: {}", e),
            LoxError::InvalidPrompt(msg) => write!(f, "invalid input: {}", msg),
            LoxError::EndOfFile => write!(f, "hit eof in the middle of parsing"),
            LoxError::CodeError(line, location, msg) => {
                write!(f, "[Line {} Error in {}]: {}", line, location, msg)
            }
        }
    }
}

impl<'a> std::error::Error for LoxError<'a> {}

impl<'a> From<io::Error> for LoxError<'a> {
    fn from(err: io::Error) -> LoxError<'a> {
        LoxError::Io(err)
    }
}

impl<'a> From<std::num::ParseFloatError> for LoxError<'a> {
    fn from(err: std::num::ParseFloatError) -> LoxError<'a> {
        LoxError::ParseFloatError(err)
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run<'a>(source: &str) -> Result<(), LoxError<'a>> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex_tokens().unwrap();

        for token in tokens {
            println!("{}", token);
        }

        Ok(())
    }

    fn run_file(&self, path: &str) -> Result<(), LoxError<'_>> {
        let source = fs::read_to_string(path)?;
        Self::run(&source)?;
        if self.had_error {
            return Err(LoxError::CodeError(0, "", ""));
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<(), LoxError<'_>> {
        loop {
            let mut input = String::new();
            print!(">> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input)?;
            if input.trim().is_empty() {
                continue;
            }
            Self::run(&input)?;
            self.had_error = false;
        }
    }
}

#[derive(Debug, Clone)]
enum TokenType {
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

pub struct Token {
    token_type: TokenType,

    lexeme: String,
    literal: Option<String>, // NOTE: String for now
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} '{}' {}",
            self.token_type,
            self.lexeme,
            self.literal.as_deref().unwrap_or("None")
        )
    }
}

impl Token {
    fn new(token_type: TokenType, lexeme: &str, literal: Option<String>, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            literal,
            line,
        }
    }
}

trait CharCheck {
    fn is_lalpha(&self) -> bool;
    fn is_lalphanumeric(&self) -> bool;
}

impl CharCheck for char {
    fn is_lalpha(&self) -> bool {
        (*self >= 'a' && *self <= 'z') || (*self >= 'A' && *self <= 'Z') || *self == '_'
    }

    fn is_lalphanumeric(&self) -> bool {
        self.is_lalpha() || self.is_numeric()
    }
}

struct Lexer {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    fn new(source: &str) -> Self {
        let keywords = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);

        Lexer {
            source: source.to_owned(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            keywords,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn lex_token<'a>(&mut self) -> Result<(), LoxError<'a>> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let amatch = self.amatch('=');
                self.add_token(if amatch {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let amatch = self.amatch('=');
                self.add_token(if amatch {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let amatch = self.amatch('=');
                self.add_token(if amatch {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let amatch = self.amatch('=');
                self.add_token(if amatch {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.amatch('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                if c.is_numeric() {
                    self.number()?;
                } else if c.is_lalpha() {
                    self.identifier();
                } else {
                    return Err(LoxError::CodeError(
                        self.line,
                        "slice of source (entire line)",
                        "encountered an unknown character or sequence of characters",
                    ));
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) {
        while self.peek().is_lalphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.add_token(token_type);
    }

    fn number<'a>(&mut self) -> Result<(), LoxError<'a>> {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let s = &self.source[self.start..self.current];
        /*
        let float_literal = match s.parse::<f64>() {
            Ok(num) => num,
            Err(e) => return Err(LoxError::ParseFloatError(e)),
        };
        */
        self.add_token_literal(TokenType::Number, Some(s.to_string()));

        Ok(())
    }

    fn string<'a>(&mut self) -> Result<(), LoxError<'a>> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::CodeError(
                self.line,
                "slice of source (entire line)",
                "unterminated string",
            ));
        }

        self.advance(); // closing "

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::LoxString, Some(value.to_string()));

        Ok(())
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            // TODO: NO UNWRAP
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            // TODO: NO UNWRAP
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn amatch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        // TODO: NO UNWRAP
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn advance(&mut self) -> char {
        // TODO: NO UNWRAP
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            literal,
            self.line,
        ))
    }

    fn lex_tokens(&mut self) -> Result<&Vec<Token>, LoxError<'_>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.lex_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));
        Ok(&self.tokens)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("usage: lox [script]");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[0]).unwrap();
    } else {
        lox.run_prompt().unwrap();
    }
}

