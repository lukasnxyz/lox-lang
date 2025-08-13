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

enum Object {
    Str(String),
    Num(f64),
    None,
}

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

// NOTE: do this with rust macros
/*
abstract class Expr {
  interface Visitor<R> {
    R visitAssignExpr(Assign expr);
    R visitBinaryExpr(Binary expr);
    R visitCallExpr(Call expr);
    R visitGetExpr(Get expr);
    R visitGroupingExpr(Grouping expr);
    R visitLiteralExpr(Literal expr);
    R visitLogicalExpr(Logical expr);
    R visitSetExpr(Set expr);
    R visitSuperExpr(Super expr);
    R visitThisExpr(This expr);
    R visitUnaryExpr(Unary expr);
    R visitVariableExpr(Variable expr);
  }

// Nested Expr classes here...
//> expr-assign
  static class Assign extends Expr {
    Assign(Token name, Expr value) {
      this.name = name;
      this.value = value;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitAssignExpr(this);
    }

    final Token name;
    final Expr value;
  }
//< expr-assign
//> expr-binary
  static class Binary extends Expr {
    Binary(Expr left, Token operator, Expr right) {
      this.left = left;
      this.operator = operator;
      this.right = right;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitBinaryExpr(this);
    }

    final Expr left;
    final Token operator;
    final Expr right;
  }
//< expr-binary
//> expr-call
  static class Call extends Expr {
    Call(Expr callee, Token paren, List<Expr> arguments) {
      this.callee = callee;
      this.paren = paren;
      this.arguments = arguments;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitCallExpr(this);
    }

    final Expr callee;
    final Token paren;
    final List<Expr> arguments;
  }
//< expr-call
//> expr-get
  static class Get extends Expr {
    Get(Expr object, Token name) {
      this.object = object;
      this.name = name;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitGetExpr(this);
    }

    final Expr object;
    final Token name;
  }
//< expr-get
//> expr-grouping
  static class Grouping extends Expr {
    Grouping(Expr expression) {
      this.expression = expression;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitGroupingExpr(this);
    }

    final Expr expression;
  }
//< expr-grouping
//> expr-literal
  static class Literal extends Expr {
    Literal(Object value) {
      this.value = value;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitLiteralExpr(this);
    }

    final Object value;
  }
//< expr-literal
//> expr-logical
  static class Logical extends Expr {
    Logical(Expr left, Token operator, Expr right) {
      this.left = left;
      this.operator = operator;
      this.right = right;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitLogicalExpr(this);
    }

    final Expr left;
    final Token operator;
    final Expr right;
  }
//< expr-logical
//> expr-set
  static class Set extends Expr {
    Set(Expr object, Token name, Expr value) {
      this.object = object;
      this.name = name;
      this.value = value;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitSetExpr(this);
    }

    final Expr object;
    final Token name;
    final Expr value;
  }
//< expr-set
//> expr-super
  static class Super extends Expr {
    Super(Token keyword, Token method) {
      this.keyword = keyword;
      this.method = method;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitSuperExpr(this);
    }

    final Token keyword;
    final Token method;
  }
//< expr-super
//> expr-this
  static class This extends Expr {
    This(Token keyword) {
      this.keyword = keyword;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitThisExpr(this);
    }

    final Token keyword;
  }
//< expr-this
//> expr-unary
  static class Unary extends Expr {
    Unary(Token operator, Expr right) {
      this.operator = operator;
      this.right = right;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitUnaryExpr(this);
    }

    final Token operator;
    final Expr right;
  }
//< expr-unary
//> expr-variable
  static class Variable extends Expr {
    Variable(Token name) {
      this.name = name;
    }

    @Override
    <R> R accept(Visitor<R> visitor) {
      return visitor.visitVariableExpr(this);
    }

    final Token name;
  }
//< expr-variable

  abstract <R> R accept(Visitor<R> visitor);
}
*/

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
            },
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

