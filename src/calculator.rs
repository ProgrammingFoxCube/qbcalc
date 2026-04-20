use std::collections::VecDeque;
use std::vec;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Token {
    Number(u32),
    Operator(Operator),
    Bracket(char)
}

pub struct Equation {}

#[derive(Debug)]
pub enum Error{
    BadToken(char),
    MismatchedParenthesis
}

impl Equation {
    pub fn parse<T: AsRef<str>>(expresion: T) -> Result<Vec<Token>, Error>{

        let expresion = expresion.as_ref();

        let characters = expresion.chars();

        let mut tokens: Vec<Token> = Vec::new();

        let mut parenthesis: Vec<char> = Vec::new();


        for c in characters {

            match c {

                '0'..='9' => {

                    let digit = c as u32 - '0' as u32;


                    if let Some(Token::Number(n)) = tokens.last_mut() {

                        *n = *n * 10 + digit;

                    } else {

                        tokens.push(Token::Number(digit));

                    }

                },

                '(' => {

                    tokens.push(Token::Bracket('('));

                    parenthesis.push(c);

                }

                ')' => {

                    tokens.push(Token::Bracket(')'));

                    if let Some(p) = parenthesis.pop() {

                        if p != '(' {

                            return Err(Error::MismatchedParenthesis);

                        }

                    }

                    else{

                        return Err(Error::MismatchedParenthesis);

                    }

                }

                '+' => tokens.push(Token::Operator(Operator::Plus)),

                '-' => tokens.push(Token::Operator(Operator::Minus)),

                '*' => tokens.push(Token::Operator(Operator::Times)),

                '/' => tokens.push(Token::Operator(Operator::Divide)),

                ' ' | '\t' | '\n' | '\r' => {},

                _ => return Err(Error::BadToken(c))

            }

        }

        if parenthesis.len() > 0{

            return Err(Error::MismatchedParenthesis);

        }

        Ok(tokens)

    }

    pub fn equation(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut output_queue: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => output_queue.push(token),
                Token::Bracket('(') => operator_stack.push(token),
                Token::Bracket(')') => {
                    // Pop everything from stack to queue until we find '('
                    while let Some(top) = operator_stack.pop() {
                        if matches!(top, Token::Bracket('(')) {
                            break;
                        }
                        output_queue.push(top);
                    }
                },
                Token::Operator(_) => {
                    while let Some(top) = operator_stack.last() {
                        if matches!(top, Token::Bracket('(')) {
                            break;
                        }
                        // If top of stack has higher or equal precedence, pop it
                        if top >= &token {
                            output_queue.push(operator_stack.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(token);
                },
                (_) => {}
            }
        }

        while let Some(op) = operator_stack.pop() {
            output_queue.push(op);
        }

        output_queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        // Reverse so we can pop from the "front" of the RPN sequence
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(n) => stack.push(n as f32),
                Token::Operator(op) => {
                    // The first pop is the RIGHT operand, the second is the LEFT
                    let right = stack.pop()?;
                    let left = stack.pop()?;

                    let result = match op {
                        Operator::Plus => left + right,
                        Operator::Minus => left - right,
                        Operator::Times => left * right,
                        Operator::Divide => left / right,
                    };
                    stack.push(result);
                }
                _ => {} // Brackets are gone by the time we hit evaluation
            }
        }

        stack.pop()
    }
}



