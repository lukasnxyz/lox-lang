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
    EOF,
    CodeError(usize, &'a str, &'a str), // line, where, msg
}

impl<'a> fmt::Display for LoxError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Io(e) => write!(f, "io error: {}", e),
            LoxError::ParseFloatError(e) => write!(f, "parse float error: {}", e),
            LoxError::InvalidPrompt(msg) => write!(f, "invalid input: {}", msg),
            LoxError::EOF => write!(f, "hit eof in the middle of parsing"),
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

    fn error(token: &Token, msg: &str) {
        if token.token_type == TokenType::Eof {
            Self::report_error(token.line, " at end", msg);
        } else {
            Self::report_error(token.line, &format!(" at '{}'", token.lexeme), msg);
        }
    }

    fn report_error(line: usize, lexeme_where: &str, msg: &str) {
        println!(
            "\x1b[31merror: \x1b[0m {}\n  -->{}: {}",
            msg,
            line,
            lexeme_where,
        );
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Clone)]
enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    None,
}

#[derive(Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
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
    fn new(token_type: TokenType, lexeme: &str, literal: Object, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            literal,
            line,
        }
    }
}

// TODO: ideally make this a macro so I can dynamically just define the grammer in a string and
//  have it expand to this
enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    fn accept(&mut self) -> String {
        let out = match self {
            Expr::Assign { name, value } => "".to_string(),
            Expr::Binary {
                left,
                operator,
                right,
            } => Self::parenthesize(&operator.lexeme, &mut [left, right]),
            Expr::Call {
                callee, arguments, ..
            } => "".to_string(),
            Expr::Get { object, name } => "".to_string(),
            Expr::Grouping { expression } => Self::parenthesize("group", &mut [expression]),
            Expr::Literal { value } => match value {
                Object::Str(s) => s.to_string(),
                Object::Num(n) => n.to_string(),
                Object::Bool(b) => b.to_string(),
                Object::None => "None".to_string(),
            },
            Expr::Logical {
                left,
                operator,
                right,
            } => "".to_string(),
            Expr::Set {
                object,
                name,
                value,
            } => "".to_string(),
            Expr::Super { keyword, method } => "".to_string(),
            Expr::This { keyword } => "".to_string(),
            Expr::Unary { operator, right } => Self::parenthesize(&operator.lexeme, &mut [right]),
            Expr::Variable { name } => "".to_string(),
        };

        "".to_string()
    }

    fn parenthesize(name: &str, exprs: &mut [&mut Expr]) -> String {
        let mut ret_str = String::new();
        ret_str.push('(');
        ret_str.push_str(name);

        for expr in exprs {
            ret_str.push(' ');
            // NOTE: this could error becuase need expr.accept(self)
            ret_str.push_str(&expr.accept());
        }

        ret_str.push(')');
        ret_str
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
        let c = self.advance()?;
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
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let amatch = self.amatch('=')?;
                self.add_token(if amatch {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '/' => {
                if self.amatch('/')? {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
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
                    self.identifier()?;
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

    fn identifier<'a>(&mut self) -> Result<(), LoxError<'a>> {
        while self.peek()?.is_lalphanumeric() {
            self.advance()?;
        }

        let text = &self.source[self.start..self.current];
        let token_type = self
            .keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier)
            .clone();

        self.add_token(token_type);

        Ok(())
    }

    fn number<'a>(&mut self) -> Result<(), LoxError<'a>> {
        while self.peek()?.is_numeric() {
            self.advance()?;
        }

        if self.peek()? == '.' && self.peek_next()?.is_numeric() {
            self.advance()?;

            while self.peek()?.is_numeric() {
                self.advance()?;
            }
        }

        let s = &self.source[self.start..self.current];
        let float_literal = match s.parse::<f64>() {
            Ok(num) => num,
            Err(e) => return Err(LoxError::ParseFloatError(e)),
        };
        self.add_token_literal(TokenType::Number, Object::Num(float_literal));

        Ok(())
    }

    fn string<'a>(&mut self) -> Result<(), LoxError<'a>> {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(LoxError::CodeError(
                self.line,
                "slice of source (entire line)",
                "unterminated string",
            ));
        }

        self.advance()?; // closing "

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::LoxString, Object::Str(value.to_string()));

        Ok(())
    }

    fn peek<'a>(&self) -> Result<char, LoxError<'a>> {
        if self.is_at_end() {
            Ok('\0')
        } else {
            match self.source.chars().nth(self.current) {
                Some(c) => Ok(c),
                None => Err(LoxError::EOF),
            }
        }
    }

    fn peek_next<'a>(&self) -> Result<char, LoxError<'a>> {
        if self.current + 1 >= self.source.len() {
            Ok('\0')
        } else {
            match self.source.chars().nth(self.current + 1) {
                Some(c) => Ok(c),
                None => Err(LoxError::EOF),
            }
        }
    }

    fn amatch<'a>(&mut self, expected: char) -> Result<bool, LoxError<'a>> {
        if self.is_at_end() {
            return Ok(false);
        }

        match self.source.chars().nth(self.current) {
            Some(curr_char) if curr_char == expected => {
                self.current += 1;
                Ok(true)
            }
            Some(_) => Ok(false),
            None => Err(LoxError::EOF),
        }
    }

    fn advance<'a>(&mut self) -> Result<char, LoxError<'a>> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        match c {
            Some(c) => Ok(c),
            None => Err(LoxError::EOF),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Object::None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Object) {
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
            .push(Token::new(TokenType::Eof, "", Object::None, self.line));
        Ok(&self.tokens)
    }
}

enum ParseError<'a> {
    Error,
    NError(&'a str),
    GenError(usize, &'a str), // line, msg
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// TODO: a visualizeable computation graph for this would be very cool
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// expression     → equality ;
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.amatch(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn amatch(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        // TODO: no clone here and use .get()
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        // TODO: no clone here and use .get()
        self.tokens[self.current - 1].clone()
    }

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.amatch(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.amatch(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.amatch(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// unary          → ( "!" | "-" ) unary
    fn unary(&mut self) -> Expr {
        if self.amatch(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            Expr::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.primary()
        }
    }

    /// primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr, ParseError>> {
        if self.amatch(&[TokenType::False]) {
            Expr::Literal {
                value: Object::Bool(false),
            }
        } else if self.amatch(&[TokenType::True]) {
            Expr::Literal {
                value: Object::Bool(true),
            }
        } else if self.amatch(&[TokenType::Nil]) {
            Expr::Literal {
                value: Object::None,
            }
        } else if self.amatch(&[TokenType::Number, TokenType::LoxString]) {
            Expr::Literal {
                value: self.previous().literal, // TODO: Object converts to Expr??
            }
        } else {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "expect ')' after expression");
            Expr::Grouping {
                expression: Box::new(expr),
            }
        }
    }

    fn consume<'a>(&mut self, token_type: TokenType, msg: &'a str) -> Result<Token, ParseError<'a>> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::GenError(self.peek().line, msg))
        }
    }

    fn error<'a>(&self, token: Token, msg: &'a str) -> ParseError<'a> {
        Lox::error(&token, msg);
        ParseError::Error
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("usage: lox [script], or lox (for repl)");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[0]).unwrap();
    } else {
        lox.run_prompt().unwrap();
    }
}
