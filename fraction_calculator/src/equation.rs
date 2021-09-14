use super::Fraction;
use std::str::FromStr;
use std::fmt::{Result as FmtResult, Formatter, Display};

// https://en.wikipedia.org/wiki/Shunting-yard_algorithm
// https://en.wikipedia.org/wiki/Reverse_Polish_notation

// TODO: Make error more descriptive
#[derive(Debug)]
struct ParseEquationError;

#[derive(Debug, Copy, Clone)]
enum OperatorType {
    Add,
    Sub,
    Mul,
    Div,
}

impl OperatorType {
    pub fn precedence(&self) -> i32 {
        match self {
            Self::Add => 0,
            Self::Sub => 0,
            Self::Mul => 1,
            Self::Div => 1,
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
        }
    }
}

impl Display for OperatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Number(Fraction),
    Operator(OperatorType),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Number(fraction) => write!(f, "{}", fraction),
            Self::Operator(operator_type) => write!(f, "{}", operator_type),
        }
    }
}

/// Only supports integers, fractions, +, -, *, /
/// Does not support parenthesis, functions, or other operators
pub struct Equation;

impl Equation {
    pub fn eval(input: &str) {
        let tokens = Self::tokenize(input);
        let rpn = Self::shunting_yard_algorithm(&tokens).unwrap();
        match Self::evaluate_rpn(&rpn) {
            Ok(mut num) => {
                num.simplify();
                println!("{} = {}", input, num)
            },
            Err(_) => panic!("ParseEquationError") // TODO: More graceful error
        };
    }

    fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        for token in input.split(' ') {
            match token {
                "+" => tokens.push(Token::Operator(OperatorType::Add)),
                "-" => tokens.push(Token::Operator(OperatorType::Sub)),
                "*" => tokens.push(Token::Operator(OperatorType::Mul)),
                "/" => tokens.push(Token::Operator(OperatorType::Div)),
                _ => {
                    let fraction = Fraction::from_str(token).unwrap(); // TODO: Error handling
                    tokens.push(Token::Number(fraction));
                }
            };
        }
        tokens
    }

    // TODO: Use Token references
    fn shunting_yard_algorithm(tokens: &Vec<Token>) -> Result<Vec<Token>, ParseEquationError> {
        let mut output_queue: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        for token in tokens {
            match token {
                Token::Number(_) => output_queue.push(*token),
                Token::Operator(current_operator) => {
                    while let Some(Token::Operator(top_operator)) = operator_stack.last() {
                        if current_operator.precedence() > top_operator.precedence() {
                            // If the current_operator has higher precedence, then just place it
                            // onto the operator_stack
                            break;
                        } else if let Some(top_token) = operator_stack.pop() {
                            // If the current_operator has lower precedence, then push the operator
                            // on the top of the stack to the output_queue
                            output_queue.push(top_token);
                        } else {
                            return Err(ParseEquationError); // Something went wrong
                        }
                    }
                    operator_stack.push(*token); // Always push the current_operator onto the stack
                },
            }
        }

        // At this point the operator_stack should be sorted by highest precedence to lowest
        while operator_stack.len() > 0 {
            if let Some(top_token) = operator_stack.pop() {
                output_queue.push(top_token);
            }
        }

        Ok(output_queue)
    }

    fn evaluate_rpn(postfix: &Vec<Token>) -> Result<Fraction, ParseEquationError>{
        let mut number_stack: Vec<Token> = Vec::new();

        for token in postfix {
            match token {
                Token::Number(_) => number_stack.push(*token),
                Token::Operator(operator) => {
                    if let Some(Token::Number(num2)) = number_stack.pop() {
                        if let Some(Token::Number(num1)) = number_stack.pop() {
                            let result: Fraction = match operator {
                                OperatorType::Add => num1 + num2,
                                OperatorType::Sub => num1 - num2,
                                OperatorType::Mul => num1 * num2,
                                OperatorType::Div => num1 / num2,
                            };
                            number_stack.push(Token::Number(result));
                        } else {
                            return Err(ParseEquationError);
                        }
                    } else {
                        return Err(ParseEquationError);
                    }
                }
            }
        }

        if let Some(Token::Number(num)) = number_stack.pop() {
            Ok(num)
        } else {
            Err(ParseEquationError)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::equation::{Equation, Token, OperatorType};
    use super::Fraction;
    use itertools::Itertools;

    #[test]
    fn rpn1() {
        let tokens = vec![
            Token::Number(Fraction::new(3, 1)),
            Token::Operator(OperatorType::Add),
            Token::Number(Fraction::new(4, 1)),
        ];
        let result = Equation::shunting_yard_algorithm(&tokens).unwrap().iter().join(" ");
        let expected = "3 4 +";
        assert_eq!(result, expected);
    }

    #[test]
    fn rpn2() {
        let tokens = vec![
            Token::Number(Fraction::new(2, 3)),
            Token::Operator(OperatorType::Add),
            Token::Number(Fraction::new(5, 8)),
            Token::Operator(OperatorType::Mul),
            Token::Number(Fraction::new(-8, 7)),
        ];
        let result = Equation::shunting_yard_algorithm(&tokens).unwrap().iter().join(" ");
        let expected = "2/3 5/8 -8/7 * +";
        assert_eq!(result, expected);
    }
}