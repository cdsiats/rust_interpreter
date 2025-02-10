use std::collections::HashMap;

use crate::{ast::{Expr, Statement}, tokens::Token};

pub struct Evaluator {
    pub env: HashMap<String, f64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }


    pub fn eval_statement(&mut self, stmt: &Statement) -> Option<f64> {
        match stmt {
            Statement::Let(var, expr) => {
                let value = self.eval_expression(expr)?;
                self.env.insert(var.clone(), value);
                None
            }
            Statement::Expression(expr) => self.eval_expression(expr),
        }
        
    }

    pub fn eval_expression(&self, expr: &Expr) -> Option<f64> {
        match expr {
            Expr::Number(n) => Some(*n),
            Expr::Identifier(name) => self.env.get(name).copied(),
            Expr::BinaryOp { left, op, right } => {
                let left_val = self.eval_expression(left)?;
                let right_val = self.eval_expression(right)?;
                match op {
                    Token::Plus => Some(left_val + right_val),
                    Token::Minus => Some(left_val - right_val),
                    Token::Star => Some(left_val * right_val),
                    Token::Slash => Some(left_val / right_val),
                    _ => None,
                }
            }
        }
    }
}