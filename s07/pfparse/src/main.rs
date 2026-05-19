// chapter-6/src/prefix_expression_parser.rs

use std::collections::VecDeque;
use std::fmt;

// Define the AST nodes
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Plus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>)
}

impl Expr {
    // Evaluate the expression
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Plus(left, right) => left.eval() + right.eval(),  // Expr::Plus(Expr::Number(4),
                                                                    // Expr::Number(3))
            Expr::Times(left, right) => left.eval() * right.eval(),
            Expr::Minus(left, right) => left.eval() - right.eval(),
            Expr::Division(left, right) => left.eval() / right.eval(),
        }
    }

    // Perform an inorder traversal and return as a string
    pub fn inorder(&self) -> String {
        match self {
            Expr::Number(n) => n.to_string(),
            Expr::Plus(left, right) => format!("({} + {})", left.inorder(), right.inorder()),
            Expr::Times(left, right) => format!("({} * {})", left.inorder(), right.inorder()),
            Expr::Minus(left, right) => format!("({} - {})", left.inorder(), right.inorder()),
            Expr::Division(left, right) => format!("({} / {}", left.inorder(), right.inorder())
        }
    }
}

// Parser function (equivalent to Python's E function)
pub fn parse_expression(q: &mut VecDeque<String>) -> Result<Expr, String> {
    if q.is_empty() {
        return Err("Invalid Prefix Expression: Unexpected end of input".to_string());
    }

    let token = q.pop_front().ok_or("Invalid Prefix Expression: Expected token")?;

    match token.as_str() {
        "+" => {
            let left = parse_expression(q).map(Box::new)?;
            let right = parse_expression(q).map(Box::new)?;
            Ok(Expr::Plus(left, right))
        }
        "*" => {
            let left = parse_expression(q).map(Box::new)?;
            let right = parse_expression(q).map(Box::new)?;
            Ok(Expr::Times(left, right))
        }
        "-" => {
            let left = parse_expression(q).map(Box::new)?;
            let right = parse_expression(q).map(Box::new)?;
            Ok(Expr::Minus(left, right))
        }
        "/" => {
            let left = parse_expression(q).map(Box::new)?;
            let right = parse_expression(q).map(Box::new)?;
            Ok(Expr::Division(left, right))
        }
        _ => {
            token.parse::<f64>()
                .map(Expr::Number)
                .map_err(|_| format!("Invalid Prefix Expression: Expected number, got '{}'", token))
        }
    }
}

fn main() {
    println!("Please enter a prefix expression (e.g., + * 2 3 5): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    let mut queue = VecDeque::from(tokens);

    match parse_expression(&mut queue) {
        Ok(root) => {
            println!("Evaluation: {}", root.eval());
            println!("Inorder Traversal: {}", root.inorder());
        }
        Err(e) => {
            eprintln!("Error parsing expression: {}", e);
        }
    }
}
