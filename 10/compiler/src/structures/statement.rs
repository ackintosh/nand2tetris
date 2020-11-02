use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::Token;

#[derive(Debug)]
pub struct Statements {
    statements: Vec<Statement>
}

impl Statements {
    pub fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let mut statements = vec![];

        Ok(Self {
            statements,
        })
    }
}

#[derive(Debug)]
struct Statement {

}
