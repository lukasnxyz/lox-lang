use crate::{
    expression::Expr,
    token::{Object, Token},
};

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
        else_branch: Box<Stmt>,
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
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> T;

    /*
    fn visit_class_stmt(&mut self, name: &Token, superclass: &Expr, methods: &Vec<Stmt>) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> T;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Stmt) -> T;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> T;
    */
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            _ => visitor.visit_expression_stmt(&Expr::Literal {
                value: Object::None,
            }),
            /*
            Stmt::Class { name, superclass, methods } => {}
            Stmt::Function { name, params, body } =>
            Stmt::If { condition, then_branch, else_branch } => {}
            Stmt::Return { keyword, value } =>
            Stmt::While { condition, body } => {}
            */
        }
    }
}
