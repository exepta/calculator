use log::error;

/** Enum for handling math operators like + - * / */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operators {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVIDED
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operators),
    Bracket(char)
}

pub struct Calculator {  }

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchParens
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();


        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    },
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchParens);
                        }
                    } else {
                        return Err(Error::MismatchParens);
                    }
                },
                '+' => tokens.push(Token::Op(Operators::ADDITION)),
                '-' => tokens.push(Token::Op(Operators::SUBTRACTION)),
                '*' => tokens.push(Token::Op(Operators::MULTIPLICATION)),
                '/' => tokens.push(Token::Op(Operators::DIVIDED)),
                ' ' => {},
                '\n' => {},
                _ => return Err(Error::BadToken(c))
            }
        }

        if parens.len() > 0 {
            return Err(Error::MismatchParens);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {}
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }

        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operators::ADDITION) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                },
                Token::Op(Operators::SUBTRACTION) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                },
                Token::Op(Operators::MULTIPLICATION) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                },
                Token::Op(Operators::DIVIDED) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                },
                _ => {}
            }
        }

        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }

    // Function for simple calculation. Returned the result from the calc way.
    pub fn calc(way: &str) -> f32 {
        let parse = Self::parse(way);
        if parse.is_err() {
            error!("{:?}", parse.err().unwrap());
            return 0.0;
        }
        let eva = Self::expression(parse.unwrap());
        Self::evaluate(eva).unwrap()
    }
}