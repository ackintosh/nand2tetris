use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::Token;
use crate::structures::class::VarName;
use crate::compilation_engine::expect_symbol;
use crate::structures::expression::{Expression, SubroutineCall};
use crate::Xml;

const STATEMENT_DEC: [&str; 5] = [
    "let",
    "if",
    "while",
    "do",
    "return",
];

/////////////////////////////////////////////////////////////
// statementsの構文
// statement*
/////////////////////////////////////////////////////////////
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
                _ => { break }
            }
        }

        Ok(Self {
            statements,
        })
    }
}

impl Xml for Statements {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<statements>\n");

        for s in &self.statements {
            xml.push_str(s.xml().as_str());
        }

        xml.push_str("</statements>\n");
        xml
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

impl Xml for Statement {
    fn xml(&self) -> String {
        match self {
            Self::Let(s) => s.xml(),
            Self::If(s) => s.xml(),
            Self::While(s) => s.xml(),
            Self::Do(s) => s.xml(),
            Self::Return(s) => s.xml(),
        }
    }
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
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
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

impl Xml for LetStatement {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<letStatement>\n");
        xml.push_str(Token::Keyword("let".into()).xml().as_str());
        xml.push_str(self.var_name.xml().as_str());

        if let Some(expression) = &self.expression_for_bracket {
            xml.push_str(Token::Symbol("[".into()).xml().as_str());
            xml.push_str(expression.xml().as_str());
            xml.push_str(Token::Symbol("]".into()).xml().as_str());
        }

        xml.push_str(Token::Symbol("=".into()).xml().as_str());
        xml.push_str(self.expression.xml().as_str());
        xml.push_str(Token::Symbol(";".into()).xml().as_str());
        xml.push_str("</letStatement>\n");
        xml
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
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
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

impl Xml for IfStatement {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<ifStatement>\n");
        xml.push_str(Token::Keyword("if".into()).xml().as_str());
        xml.push_str(Token::Symbol("(".into()).xml().as_str());
        xml.push_str(self.expression.xml().as_str());
        xml.push_str(Token::Symbol(")".into()).xml().as_str());

        xml.push_str(Token::Symbol("{".into()).xml().as_str());
        xml.push_str(self.statements.xml().as_str());
        xml.push_str(Token::Symbol("}".into()).xml().as_str());

        if let Some(else_statements) = &self.else_statements {
            xml.push_str(Token::Keyword("else".into()).xml().as_str());
            xml.push_str(Token::Symbol("{".into()).xml().as_str());
            xml.push_str(else_statements.xml().as_str());
            xml.push_str(Token::Symbol("}".into()).xml().as_str());
        }

        xml.push_str("</ifStatement>\n");
        xml
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
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let _ = expect_symbol("(", iter.next().unwrap());
        let expression = Expression::extract(iter)?;
        let _ = expect_symbol(")", iter.next().unwrap());
        let _ = expect_symbol("{", iter.next().unwrap());
        let statements = Statements::extract(iter)?;
        let _ = expect_symbol("}", iter.next().unwrap());

        Ok(Self { expression, statements })
    }
}

impl Xml for WhileStatement {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<whileStatement>\n");
        xml.push_str(Token::Keyword("while".into()).xml().as_str());
        xml.push_str(Token::Symbol("(".into()).xml().as_str());
        xml.push_str(self.expression.xml().as_str());
        xml.push_str(Token::Symbol(")".into()).xml().as_str());
        xml.push_str(Token::Symbol("{".into()).xml().as_str());
        xml.push_str(self.statements.xml().as_str());
        xml.push_str(Token::Symbol("}".into()).xml().as_str());
        xml.push_str("</whileStatement>\n");
        xml
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
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let token = iter.next().unwrap();
        let subroutine_call = SubroutineCall::extract_with_first_token(token, iter)?;
        let _ = expect_symbol(";", iter.next().unwrap())?;
        Ok(Self { subroutine_call })
    }
}

impl Xml for DoStatement {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<doStatement>\n");
        xml.push_str(Token::Keyword("do".into()).xml().as_str());
        xml.push_str(self.subroutine_call.xml().as_str());
        xml.push_str(Token::Symbol(";".into()).xml().as_str());
        xml.push_str("</doStatement>\n");
        xml
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
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let expression = {
            match iter.peek().unwrap() {
                Token::Symbol(symbol) => {
                    if symbol == ";" {
                        None
                    } else {
                        Some(Expression::extract(iter)?)
                    }
                }
                _ => Some(Expression::extract(iter)?)
            }
        };
        let _ = expect_symbol(";", iter.next().unwrap())?;
        Ok(Self { expression })
    }
}

impl Xml for ReturnStatement {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<returnStatement>\n");
        xml.push_str(Token::Keyword("return".into()).xml().as_str());

        if let Some(expression) = &self.expression {
            xml.push_str(expression.xml().as_str());
        }

        xml.push_str(Token::Symbol(";".into()).xml().as_str());
        xml.push_str("</returnStatement>\n");
        xml
    }
}
