use crate::tokenizer::Token;
use crate::structures::Class;

pub struct CompilationEngine {
}

impl CompilationEngine {
    pub fn compile(tokens: Vec<Token>) -> Result<Class, String> {
        let mut iter = tokens.iter().peekable();
        Ok(Class::extract(&mut iter)?)
    }

}

pub fn expect_symbol(expected: &str, token: &Token) -> Result<(), String> {
    match token {
        Token::Symbol(symbol) => {
            if symbol == expected {
                Ok(())
            } else {
                Err(format!("expected `{}`, but {:?} has passed", expected, token))
            }
        }
        _ => Err(format!("expected `{}`, but {:?} has passed", expected, token))
    }
}
