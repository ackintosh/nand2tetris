use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenizer::Token;
use crate::structures::class::{VarName, SubroutineName};
use crate::compilation_engine::expect_symbol;
use crate::Xml;

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
    pub fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
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

impl Xml for Expression {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<expression>\n");

        xml.push_str(self.term.xml().as_str());

        for (token, term) in &self.op_terms {
            xml.push_str(token.xml().as_str());
            xml.push_str(term.xml().as_str());
        }

        xml.push_str("</expression>\n");
        xml
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
    IntegerConstant(String),
    StringConstant(String),
    KeywordConstant(Token),
    VarName(VarName),
    VarNameWithExpression(VarName, Expression),
    SubroutineCall(SubroutineCall),
    Expression(Expression),
    UnaryOp(Token, Box<Term>), // 再帰構造なのでBoxで包む
}

impl Term {
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let token = iter.next().unwrap();
        Ok(match token {
            Token::IntegerConst(integer) => Term::IntegerConstant(integer.into()),
            Token::StringConst(string) => Term::StringConstant(string.into()),
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

impl Xml for Term {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<term>\n");

        match self {
            Self::IntegerConstant(integer) => xml.push_str(format!("<integerConstant>{}</integerConstant>\n", integer).as_str()),
            Self::StringConstant(string) => xml.push_str(format!("<stringConstant>{}</stringConstant>\n", string).as_str()),
            Self::KeywordConstant(token) => xml.push_str(token.xml().as_str()),
            Self::VarName(var_name) => xml.push_str(var_name.xml().as_str()),
            Self::VarNameWithExpression(var_name, expression) => {
                xml.push_str(var_name.xml().as_str());
                xml.push_str(expression.xml().as_str());
            }
            Self::SubroutineCall(subroutine_call) => xml.push_str(subroutine_call.xml().as_str()),
            Self::Expression(expression) => {
                xml.push_str(Token::Symbol("(".into()).xml().as_str());
                xml.push_str(expression.xml().as_str());
                xml.push_str(Token::Symbol(")".into()).xml().as_str());
            },
            Self::UnaryOp(token, term) => {
                xml.push_str(token.xml().as_str());
                xml.push_str(term.xml().as_str());
            }
        }

        xml.push_str("</term>\n");
        xml
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
    pub fn extract_with_first_token(token: &Token, iter: &mut Peekable<Iter<Token>>) -> Result<SubroutineCall, String> {
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

impl Xml for SubroutineCall {
    fn xml(&self) -> String {
        let mut xml = String::new();

        match self {
            Self::Subroutine(token, expression_list) => {
                xml.push_str(token.xml().as_str());
                xml.push_str(Token::Symbol("(".into()).xml().as_str());
                xml.push_str(expression_list.xml().as_str());
                xml.push_str(Token::Symbol(")".into()).xml().as_str());
            }
            Self::Method(token, subroutine_name, expression_list) => {
                xml.push_str(token.xml().as_str());
                xml.push_str(Token::Symbol(".".into()).xml().as_str());
                xml.push_str(subroutine_name.xml().as_str());
                xml.push_str(Token::Symbol("(".into()).xml().as_str());
                xml.push_str(expression_list.xml().as_str());
                xml.push_str(Token::Symbol(")".into()).xml().as_str());
            }
        }

        xml
    }
}

/////////////////////////////////////////////////////////////
// expressionListの構文
// (expression (`,` expression)* )?
/////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct ExpressionList {
    expressions: Option<Vec<Expression>>,
}

impl ExpressionList {
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
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

    fn should_extract_expression(iter: &mut Peekable<Iter<Token>>) -> bool {
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

impl Xml for ExpressionList {
    fn xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<expressionList>\n");

        if let Some(expressions) = &self.expressions {
            for (i, expression) in expressions.iter().enumerate() {
                if i > 0 {
                    xml.push_str(Token::Symbol(",".into()).xml().as_str());
                }
                xml.push_str(expression.xml().as_str());
            }
        }

        xml.push_str("</expressionList>\n");
        xml
    }
}
