use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::Token;
use crate::structures::class::VarName;
use crate::compilation_engine::expect_symbol;
use crate::structures::expression::{Expression, SubroutineCall};

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
                        "if" => Statement::If(IfStatement::extract(iter)?),
                        "while" => Statement::While(WhileStatement::extract(iter)?),
                        "do" => Statement::Do(DoStatement::extract(iter)?),
                        "return" => Statement::Return(ReturnStatement::extract(iter)?),
                        _ => { break; }
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
    If(IfStatement),
    While(WhileStatement),
    Do(DoStatement),
    Return(ReturnStatement),
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

/////////////////////////////////////////////////////////////
// ifStatementの構文
// `if` `(` expression `)` `{` statements `}`
// (`else` `{` statements `}` )?
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct IfStatement {
    expression: Expression,
    statements: Statements,
    else_statements: Option<Statements>,
}

impl IfStatement {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let _ = expect_symbol("(", iter.next().unwrap());
        let expression = Expression::extract(iter)?;
        let _ = expect_symbol(")", iter.next().unwrap());
        let _ = expect_symbol("{", iter.next().unwrap());
        let statements = Statements::extract(iter)?;
        let _ = expect_symbol("}", iter.next().unwrap());
        let else_statements = {
            match iter.peek().unwrap() {
                Token::Keyword(keyword) => {
                    if keyword == "else" {
                        let _ = iter.next().unwrap();
                        let _ = expect_symbol("{", iter.next().unwrap());
                        let statements = Statements::extract(iter)?;
                        let _ = expect_symbol("}", iter.next().unwrap());
                        Some(statements)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        };

        Ok(Self {
            expression,
            statements,
            else_statements,
        })
    }
}
/////////////////////////////////////////////////////////////
// whileStatementの構文
// `while` `(` expression `)` `{` statements `}`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct WhileStatement {
    expression: Expression,
    statements: Statements,
}

impl WhileStatement {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let _ = expect_symbol("(", iter.next().unwrap());
        let expression = Expression::extract(iter)?;
        let _ = expect_symbol(")", iter.next().unwrap());
        let _ = expect_symbol("{", iter.next().unwrap());
        let statements = Statements::extract(iter)?;
        let _ = expect_symbol("}", iter.next().unwrap());

        Ok(Self { expression, statements })
    }
}

/////////////////////////////////////////////////////////////
// doStatementの構文
// `do` subroutineCall `;`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct DoStatement {
    subroutine_call: SubroutineCall,
}

impl DoStatement {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let token = iter.next().unwrap();
        let subroutine_call = SubroutineCall::extract_with_first_token(token, iter)?;
        Ok(Self { subroutine_call })
    }
}

/////////////////////////////////////////////////////////////
// returnStatementの構文
// `return` expression? `;`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct ReturnStatement {
    expression: Option<Expression>,
}

impl ReturnStatement {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let expression = {
            match iter.peek().unwrap() {
                Token::Symbol(symbol) => {
                    if symbol == ";" {
                        let _ = iter.next().unwrap();
                        None
                    } else {
                        Some(Expression::extract(iter)?)
                    }
                }
                _ => Some(Expression::extract(iter)?)
            }
        };
        Ok(Self { expression })
    }
}
