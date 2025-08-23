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
    r#None,
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

#[derive(Clone, Debug, PartialOrd)]
pub enum Object {
    r#String(String),
    Number(f64),
    Bool(bool),
    None,
}

// TODO: impl PartialOrd for Object custom to define the exact behaviour

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::String(s) => s.to_string(),
                Object::Number(n) => n.to_string(),
                Object::Bool(b) => b.to_string(),
                Object::None => "none".to_string(),
            }
        )
    }
}

/// isEqual()
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
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
            Object::String(val) => Some(val.to_string()),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        match self {
            Object::String(_) => true,
            _ => false,
        }
    }

    pub fn to_num(&self) -> Option<f64> {
        match self {
            Object::Number(val) => Some(*val),
            _ => None,
        }
    }

    pub fn is_num(&self) -> bool {
        match self {
            Object::Number(_) => true,
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
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
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

struct AstPrinter;
impl AstPrinter {
    fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}

// TODO: ideally make this a macro so I can dynamically just define the grammer in a string and
//  have it expand to this
#[derive(Clone)]
pub enum Expr {
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

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &Object) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_var_expr(&mut self, name: &Token) -> T;
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> T;
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;

    /*
    fn visit_call_expr(&mut self, callee: &Expr, arguments: &[Expr]) -> T;
    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> T;
    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> T;
    fn visit_this_expr(&mut self, keyword: &Token) -> T;
    */
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &Object) -> String {
        match value {
            Object::String(s) => s.to_string(),
            Object::Number(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::None => "None".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_var_expr(&mut self, name: &Token) -> String {
        name.lexeme.clone()
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> String {
        format!("(= {} {})", name.lexeme, value.accept(self))
    }

    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    /*
    fn visit_call_expr(&self, callee: &Expr, arguments: &[Expr]) -> String {
        let mut result = format!("(call {}", callee.accept(self));
        for arg in arguments {
            result.push(' ');
            result.push_str(&arg.accept(self));
        }
        result.push(')');
        result
    }

    fn visit_get_expr(&self, object: &Expr, name: &Token) -> String {
        format!("(. {} {})", object.accept(self), name.lexeme)
    }

    fn visit_set_expr(&self, object: &Expr, name: &Token, value: &Expr) -> String {
        format!(
            "(= (. {} {}) {})",
            object.accept(self),
            name.lexeme,
            value.accept(self)
        )
    }

    fn visit_super_expr(&self, keyword: &Token, method: &Token) -> String {
        format!("(super {})", method.lexeme)
    }

    fn visit_this_expr(&self, keyword: &Token) -> String {
        "this".to_string()
    }
    */
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_var_expr(name),
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
            _ => visitor.visit_literal_expr(&Object::None),
            /*
            Expr::Call {
                callee, arguments, ..
            } => visitor.visit_call_expr(callee, arguments),
            Expr::Get { object, name } => visitor.visit_get_expr(object, name),
            Expr::Set {
                object,
                name,
                value,
            } => visitor.visit_set_expr(object, name, value),
            Expr::Super { keyword, method } => visitor.visit_super_expr(keyword, method),
            Expr::This { keyword } => visitor.visit_this_expr(keyword),
            */
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", AstPrinter.print(self))
    }
}

#[derive(Clone)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        superclass: Expr,
        methods: Vec<Stmt>, // have to be Statement::Function
    },
    Expression {
        expression: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> T;

    /*
    fn visit_class_stmt(&mut self, name: &Token, superclass: &Expr, methods: &Vec<Stmt>) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> T;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> T;
    */
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            _ => visitor.visit_expression_stmt(&Expr::Literal {
                value: Object::None,
            }),
            /*
            Stmt::Class { name, superclass, methods } => {}
            Stmt::Function { name, params, body } =>
            Stmt::Return { keyword, value } =>
            */
        }
    }
}
