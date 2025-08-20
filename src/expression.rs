use crate::token::{Object, Token};
use std::fmt;

struct AstPrinter;
impl AstPrinter {
    fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
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
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&self, expression: &Expr) -> T;
    fn visit_literal_expr(&self, value: &Object) -> T;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> T;
    fn visit_variable_expr(&self, name: &Token) -> T;

    /*
    fn visit_assign_expr(&self, name: &Token, value: &Expr) -> T;
    fn visit_call_expr(&self, callee: &Expr, arguments: &[Expr]) -> T;
    fn visit_get_expr(&self, object: &Expr, name: &Token) -> T;
    fn visit_logical_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_set_expr(&self, object: &Expr, name: &Token, value: &Expr) -> T;
    fn visit_super_expr(&self, keyword: &Token, method: &Token) -> T;
    fn visit_this_expr(&self, keyword: &Token) -> T;
    */
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&self, value: &Object) -> String {
        match value {
            Object::String(s) => s.to_string(),
            Object::Number(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
            Object::None => "None".to_string(),
        }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_variable_expr(&self, name: &Token) -> String {
        name.lexeme.clone()
    }

    /*
    fn visit_assign_expr(&self, name: &Token, value: &Expr) -> String {
        format!("(= {} {})", name.lexeme, value.accept(self))
    }

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

    fn visit_logical_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
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
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            _ => visitor.visit_literal_expr(&Object::None),
            /*
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Call {
                callee, arguments, ..
            } => visitor.visit_call_expr(callee, arguments),
            Expr::Get { object, name } => visitor.visit_get_expr(object, name),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
            Expr::Set {
                object,
                name,
                value,
            } => visitor.visit_set_expr(object, name, value),
            Expr::Super { keyword, method } => visitor.visit_super_expr(keyword, method),
            Expr::This { keyword } => visitor.visit_this_expr(keyword),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
            */
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", AstPrinter.print(self))
    }
}
