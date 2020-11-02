use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::Token;
use crate::structures::class::VarName;
use crate::compilation_engine::expect_symbol;
use crate::structures::expression::Expression;

const STATEMENT_DEC: [&str; 5] = [
    "let",
    "if",
    "while",
    "do",
    "return",
];

#[derive(Debug)]
pub struct Statements {
    statements: Vec<Statement>
}

impl Statements {
    pub fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let mut statements = vec![];

        loop {
            match iter.peek().unwrap() {
                Token::Keyword(keyword) => {
                    if !STATEMENT_DEC.contains(&keyword.as_str()) {
                        break;
                    }

                    // 先読みして確認済みのキーワードをイテレータから取り出す
                    let _ = iter.next().unwrap();
                    let statement = match keyword.as_str() {
                        "let" => Statement::Let(LetStatement::extract(&mut iter)?),
                        _ => { break; } // TODO
                    };

                    statements.push(statement);
                }
                _ => { break; }
            }
        }

        Ok(Self {
            statements,
        })
    }

}

/////////////////////////////////////////////////////////////
// statementの構文
// letStatement | ifStatement | whileStatement | doStatement | returnStatement
/////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Statement {
    Let(LetStatement),
    If,
    While,
    Do,
    Return,
}

/////////////////////////////////////////////////////////////
// letStatementの構文
// `let` varName (`[` expression `]`)? `=` expression `;`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct LetStatement {
    var_name: VarName,
    expression_for_bracket: Option<Expression>,
    expression: Expression,
}

impl LetStatement {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let var_name = VarName::new(iter.next().unwrap())?;

        let expression_for_bracket = {
            match iter.peek().unwrap() {
                Token::Symbol(symbol) => {
                    if symbol == "[" {
                        let _ = expect_symbol("[", iter.next().unwrap())?;
                        let expression = Expression::extract(iter)?;
                        let _ = expect_symbol("]", iter.next().unwrap())?;
                        Some(expression)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        };

        let _ = expect_symbol("=", iter.next().unwrap())?;
        let expression = Expression::extract(iter)?;
        let _ = expect_symbol(";", iter.next().unwrap())?;

        Ok(Self {
            var_name,
            expression_for_bracket,
            expression,
        })
    }
}
