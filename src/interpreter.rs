use crate::{
    expression::{Expr, ExprVisitor},
    token::{Object, Token, TokenType},
};

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, value: &Object) -> Object {
        value.clone()
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> Object {
        expression.accept(self)
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Object {
        let right = right.accept(self);
        match operator.token_type {
            // TODO: no unwrap here plz
            TokenType::Minus => Object::Number(-right.to_num().unwrap()),
            TokenType::Bang => Object::Bool(!right.to_bool()),
            _ => Object::None,
        }
    }

    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Object {
        let left = left.accept(self);
        let right = right.accept(self);

        match operator.token_type {
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),

            TokenType::BangEqual => Object::Bool(left != right),
            TokenType::EqualEqual => Object::Bool(left == right),

            TokenType::Minus => Object::Number(left.to_num().unwrap() - right.to_num().unwrap()),
            TokenType::Plus => {
                if left.is_str() && right.is_str() {
                    Object::String(left.to_str().unwrap() + &right.to_str().unwrap())
                } else if left.is_num() && right.is_num() {
                    Object::Number(left.to_num().unwrap() + right.to_num().unwrap())
                } else {
                    panic!(); // can't add a string and a number
                }
            }

            TokenType::Slash => Object::Number(left.to_num().unwrap() / right.to_num().unwrap()),
            TokenType::Star => Object::Number(left.to_num().unwrap() * right.to_num().unwrap()),

            _ => Object::None,
        }
    }
}
