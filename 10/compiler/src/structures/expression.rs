use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::Token;
use crate::structures::class::{VarName, SubroutineName};
use crate::compilation_engine::expect_symbol;

const OP: [&str; 9] = [
    "+",
    "-",
    "*",
    "/",
    "&",
    "|",
    "<",
    ">",
    "=",
];
const UNARY_OP: [&str; 2] = [
    "-",
    "~",
];
const KEYWORD_CONSTANT: [&str; 4] = [
    "true",
    "false",
    "null",
    "this",
];

/////////////////////////////////////////////////////////////
// expressionの構文
// term (op term)*
/////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Expression {
    term: Box<Term>, // 再帰構造なのでBoxで包む
    op_terms: Vec<(Token, Term)>
}

impl Expression {
    pub fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let term = Term::extract(iter)?;
        let mut op_terms = vec![];

        loop {
            match iter.peek().unwrap() {
                Token::Symbol(symbol) => {
                    if !OP.contains(&symbol.as_str()) {
                        break;
                    }
                }
                _ => { break; }
            }

            let op = iter.next().unwrap();
            let term = Term::extract(iter)?;
            op_terms.push((op.into(), term));
        }

        Ok(Self { term: Box::new(term), op_terms })
    }
}

/////////////////////////////////////////////////////////////
// termの構文
// integerConstant | stringConstant | keywordConstant
// | varName
// | varName `[` expresion `]`
// | subroutineCall
// | `(` expression `)`
// | unaryOp term
/////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Term {
    IntegerConstant(Token),
    StringConstant(Token),
    KeywordConstant(Token),
    VarName(VarName),
    VarNameWithExpression(VarName, Expression),
    SubroutineCall(SubroutineCall),
    Expression(Expression),
    UnaryOp(Token, Box<Term>), // 再帰構造なのでBoxで包む
}

impl Term {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let token = iter.next().unwrap();
        Ok(match token {
            Token::IntegerConst(_) => Term::IntegerConstant(token.into()),
            Token::StringConst(_) => Term::StringConstant(token.into()),
            Token::Keyword(keyword) => {
                if KEYWORD_CONSTANT.contains(&keyword.as_str()) {
                    Term::KeywordConstant(token.into())
                } else {
                    return Err("invalid keyword".into())
                }
            },
            Token::Identifier(_) => {
                let next = iter.peek().unwrap();
                match next {
                    Token::Symbol(symbol) => {
                        if symbol == "[" {
                            let var_name = VarName::new(token)?;
                            let _ = expect_symbol("[", iter.next().unwrap())?;
                            let expression = Expression::extract(iter)?;
                            let _ = expect_symbol("]", iter.next().unwrap())?;
                            Term::VarNameWithExpression(var_name, expression)
                        } else if symbol == "(" || symbol == "." {
                            Term::SubroutineCall(SubroutineCall::extract_with_first_token(
                                token,
                                iter
                            )?)
                        } else {
                            Term::VarName(VarName::new(token)?)
                        }
                    }
                    _ => Term::VarName(VarName::new(token)?),
                }
            }
            Token::Symbol(symbol) => {
                if symbol == "(" {
                    let expression = Expression::extract(iter)?;
                    let _ = expect_symbol(")", iter.next().unwrap())?;
                    Term::Expression(expression)
                } else if UNARY_OP.contains(&symbol.as_str()) {
                    Term::UnaryOp(token.into(), Box::new(Term::extract(iter)?))
                } else {
                    return Err(format!("invalid symbol as term: {}", symbol).into());
                }
            }
        })
    }
}

/////////////////////////////////////////////////////////////
// subroutineCallの構文
// subroutineName `(` expressionList `)`
// | (className | varName) `.` subroutineName `(` expressionList `)`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum SubroutineCall {
    Subroutine(Token, ExpressionList),
    Method(Token, SubroutineName, ExpressionList),
}

impl SubroutineCall {
    pub fn extract_with_first_token(token: &Token, mut iter: &mut Peekable<Iter<Token>>) -> Result<SubroutineCall, String> {
        let next = iter.peek().unwrap();
        match next {
            Token::Symbol(symbol) => {
                if symbol == "(" {
                    expect_symbol("(", iter.next().unwrap())?;
                    let expression_list = ExpressionList::extract(iter)?;
                    expect_symbol(")", iter.next().unwrap())?;
                    Ok(SubroutineCall::Subroutine(
                        token.into(),
                        expression_list
                    ))
                } else if symbol == "." {
                    expect_symbol(".", iter.next().unwrap())?;
                    let subroutine_name = SubroutineName::new(iter.next().unwrap())?;
                    expect_symbol("(", iter.next().unwrap())?;
                    let expression_list = ExpressionList::extract(iter)?;
                    expect_symbol(")", iter.next().unwrap())?;
                    Ok(SubroutineCall::Method(
                        token.into(),
                        subroutine_name,
                        expression_list
                    ))
                } else {
                    Err("invalid keyword".into())
                }
            }
            _ => Err(format!("invalid subroutine call: {:?}", next).into())
        }
    }
}

/////////////////////////////////////////////////////////////
// expressionListの構文
// (expression (`,` expression)* )?
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct ExpressionList {
    expressions: Option<Vec<Expression>>,
}

impl ExpressionList {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let expressions = {
            if Self::should_extract_expression(iter) {
                let mut expressions = vec![];
                expressions.push(Expression::extract(iter)?);

                loop {
                    match iter.peek().unwrap() {
                        Token::Symbol(symbol) => {
                            if symbol != "," {
                                break;
                            }
                        }
                        _ => { break; }
                    }

                    let _ = iter.next().unwrap();
                    expressions.push(Expression::extract(iter)?);
                }

                Some(expressions)
            } else {
                None
            }
        };

        Ok(Self { expressions })
    }

    fn should_extract_expression(mut iter: &mut Peekable<Iter<Token>>) -> bool {
        match iter.peek().unwrap() {
            Token::Symbol(symbol) => {
                if symbol == ")" {
                    // expression無し
                    false
                } else {
                    true
                }
            }
            _ => true,
        }
    }
}
