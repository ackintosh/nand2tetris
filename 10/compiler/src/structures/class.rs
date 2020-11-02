use crate::tokenizer::Token;
use std::iter::Peekable;
use std::slice::Iter;
use crate::compilation_engine::expect_symbol;
use crate::structures::Statements;

const CLASS_KEYWORD: &str = "class";
const CLASS_VAR_DEC_KEYWORD: [&str; 2] = [
    "static",
    "field",
];
const CLASS_VAR_DEC_TYPE_KEYWORD: [&str; 3] = [
    "int",
    "char",
    "boolean",
];
const SUBROUTINE_DEC_KEYWORD: [&str; 3] = [
    "constructor",
    "function",
    "method",
];

/////////////////////////////////////////////////////////////
// classの構文
// `class` className `{` classVarDec* subroutineDec* `}`
/////////////////////////////////////////////////////////////
pub struct Class {
    class_name: ClassName,
    class_var_decs: Vec<ClassVarDec>,
    subroutine_decs: Vec<SubroutineDec>,
}

impl Class {
    pub fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        // `class`
        match iter.next().expect("should have keyword") {
            Token::Keyword(keyword) => {
                if keyword != CLASS_KEYWORD {
                    return Err("should start with `class` keyword".into());
                }
            }
            _ => return Err("should start with `class` keyword".into())
        }

        // className
        let class_name = ClassName::extract(&mut iter)?;

        // `{`
        expect_symbol("{", iter.next().unwrap())?;

        // classVarDec*
        let class_var_decs = Self::compile_class_var_dec(&mut iter);
        println!("class_var_decs: {:?}", class_var_decs);

        // subroutineDec*
        let subroutine_decs = Self::compile_subroutine_dec(&mut iter);
        println!("subroutine_decs: {:?}", subroutine_decs);

        // `}`

        let t = iter.peek().unwrap();
        println!("compile iter.peek(): {:?}", t);
        let t = iter.next().unwrap();
        println!("compile iter.next(): {:?}", t);

        Ok(Class{
            class_name,
            class_var_decs,
            subroutine_decs,
        })
    }

    fn compile_class_var_dec(mut iter: &mut Peekable<Iter<Token>>) -> Vec<ClassVarDec> {
        let mut class_var_decs = vec![];

        loop {
            // 先読みしてクラス変数の宣言かどうかを判定する
            let dec = iter.peek();
            if dec.is_none() {
                break;
            }
            match dec.unwrap() {
                Token::Keyword(keyword) => {
                    if !CLASS_VAR_DEC_KEYWORD.contains(&keyword.as_str()) {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
            // 宣言を確認できたのでイテレータから取り出す
            let dec = iter.next().unwrap();

            let class_var_dec_type = Type::new(iter.next().unwrap()).unwrap();

            let var_names = VarName::extract_var_names(&mut iter).unwrap();

            // ClassVarDecの宣言はセミコロンで終わる
            expect_symbol(";", iter.next().unwrap()).unwrap();

            class_var_decs.push(ClassVarDec {
                dec: dec.into(),
                r#type: class_var_dec_type,
                var_names,
            });
        }

        class_var_decs
    }

    fn compile_subroutine_dec(mut iter: &mut Peekable<Iter<Token>>) -> Vec<SubroutineDec> {
        let mut subroutine_decs = vec![];

        loop {
            // 先読みしてサブルーチンの宣言かどうかを判定する
            let dec = iter.peek();
            if dec.is_none() {
                break;
            }
            match dec.unwrap() {
                Token::Keyword(keyword) => {
                    if !SUBROUTINE_DEC_KEYWORD.contains(&keyword.as_str()) {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
            // 宣言を確認できたのでイテレータから取り出す
            let dec = iter.next().unwrap();

            let return_type = SubroutineReturnType::new(iter.next().unwrap()).unwrap();

            let subroutine_name = SubroutineName::new(iter.next().unwrap()).unwrap();

            expect_symbol("(", iter.next().unwrap()).unwrap();

            let parameter_list = ParameterList::extract(&mut iter).unwrap();

            expect_symbol(")", iter.next().unwrap()).unwrap();

            let subroutine_body = SubroutineBody::extract(&mut iter).unwrap();

            subroutine_decs.push(SubroutineDec {
                dec: dec.into(),
                return_type,
                subroutine_name,
                parameter_list,
                subroutine_body,
            });
        }

        subroutine_decs
    }
}

/////////////////////////////////////////////////////////////
// classVarDecの構文
// (`static` | `field`) type varName (`,` varName)* `;`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct ClassVarDec {
    dec: Token,
    r#type: Type,
    var_names: Vec<VarName>,
}

/////////////////////////////////////////////////////////////
// typeの構文
// `int` | `char` | `boolean` | className
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Type {
    inner: Token,
}

impl Type {
    fn new(token: &Token) -> Result<Self, String> {
        match token {
            Token::Keyword(k) => {
                if !CLASS_VAR_DEC_TYPE_KEYWORD.contains(&k.as_str()) {
                    return Err("invalid Type".into())
                }

                Ok(Self { inner: token.into() })
            }
            Token::Identifier(_) => Ok(Self { inner: token.into() }),
            _ => Err("invalid Type".into())
        }
    }
}

/////////////////////////////////////////////////////////////
// subroutineBodyの構文
// `{` varDec* statements `}`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct SubroutineBody {
    var_decs: Vec<VarDec>,
    statements: Statements,
}

impl SubroutineBody {
    fn extract(mut iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        expect_symbol("{", iter.next().unwrap());

        let var_decs = VarDec::extract_var_decs(&mut iter)?;

        let statements = Statements::extract(&mut iter)?;

        // expect_symbol("}", iter.next().unwrap());

        Ok(Self {
            var_decs,
            statements,
        })
    }
}

/////////////////////////////////////////////////////////////
// varDecの構文
// `var` type varName (`,` varName)* `;`
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct VarDec {
    r#type: Type,
    var_names: Vec<VarName>,
}

impl VarDec {
    fn extract_var_decs(mut iter: &mut Peekable<Iter<Token>>) -> Result<Vec<Self>, String> {
        let mut var_decs = vec![];

        loop {
            let token = iter.peek().unwrap();
            match token {
                Token::Keyword(keyword) => {
                    if keyword != "var" {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }

            // `var` を取り出す
            let _ = iter.next().unwrap();
            let r#type = Type::new(iter.next().unwrap())?;
            let var_names = VarName::extract_var_names(&mut iter)?;
            var_decs.push(VarDec{
                r#type,
                var_names,
            });
        }

        Ok(var_decs)
    }
}

/////////////////////////////////////////////////////////////
// classNameの構文
// identifier
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct ClassName {
    inner: Token,
}

impl ClassName {
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let token = iter.next().unwrap();
        if let Token::Identifier(_) = token {
            Ok(Self { inner: token.into() })
        } else {
            Err("class name has missed".into())
        }
    }
}

/////////////////////////////////////////////////////////////
// subroutineNameの構文
// identifier
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct SubroutineName {
    inner: Token,
}

impl SubroutineName {
    fn new(token: &Token) -> Result<Self, String> {
        match token {
            Token::Identifier(_) => Ok(Self { inner: token.into() }),
            _ => Err("invalid SubroutineName".into())
        }
    }
}

/////////////////////////////////////////////////////////////
// varNameの構文
// identifier
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct VarName {
    inner: Token,
}

impl VarName {
    fn new(token: &Token) -> Result<Self, String> {
        match token {
            Token::Identifier(_) => Ok(Self { inner: token.into() }),
            _ => Err("invalid VarName".into())
        }
    }

    fn extract_var_names(iter: &mut Peekable<Iter<Token>>) -> Result<Vec<Self>, String> {
        let mut var_names = vec![];

        // 少なくとも1つVarNameが宣言される
        var_names.push(VarName::new(iter.next().unwrap()).unwrap());
        // 2つめ以降のVarName宣言を処理する
        loop {
            // 先読みしてVarNameを組み立てるべきか判定する
            if let Token::Symbol(symbol) = iter.peek().unwrap() {
                if symbol == "," {
                    let _ = iter.next(); // 先読みして判定していた Token::Symbol(",") を捨てる
                    var_names.push(VarName::new(iter.next().unwrap()).unwrap());
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(var_names)
    }
}

/////////////////////////////////////////////////////////////
// subroutineDecの構文
// (`constructor` | `function` | `method`) (`void` | type) subroutineName `(` parameterList `)` subroutineBody
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct SubroutineDec {
    dec: Token,
    return_type: SubroutineReturnType,
    subroutine_name: SubroutineName,
    parameter_list: ParameterList,
    subroutine_body: SubroutineBody,
}

#[derive(Debug)]
enum SubroutineReturnType {
    Void(Token),
    Type(Type),
}

impl SubroutineReturnType {
    fn new(token: &Token) -> Result<Self, String> {
        match token {
            Token::Keyword(k) => {
                if k == "void" {
                    Ok(SubroutineReturnType::Void(token.into()))
                } else {
                    Err("invalid return type".into())
                }
            }
            _ => {
                Ok(SubroutineReturnType::Type(Type::new(token)?))
            }
        }
    }
}

/////////////////////////////////////////////////////////////
// parameterListの構文
// ((type varName) (`,` type varName)* )?
/////////////////////////////////////////////////////////////
#[derive(Debug)]
struct ParameterList {
    list: Vec<Parameter>,
}

impl ParameterList {
    fn extract(iter: &mut Peekable<Iter<Token>>) -> Result<Self, String> {
        let mut list = vec![];
        list.push(
            Parameter::new(
                iter.next().unwrap(),
                iter.next().unwrap()
            )?
        );

        loop {
            // 先読みしてパラメータの宣言が続くかどうかを判定する
            match iter.peek().unwrap() {
                Token::Symbol(symbol) => {
                    if symbol != "," {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }

            expect_symbol(",", iter.next().unwrap());
            list.push(
                Parameter::new(
                    iter.next().unwrap(),
                    iter.next().unwrap()
                )?
            );
        }

        Ok(Self { list })
    }
}

#[derive(Debug)]
struct Parameter {
    r#type: Type,
    var_name: VarName,
}

impl Parameter {
    fn new(type_token: &Token, var_name_token: &Token) -> Result<Self, String> {
        let r#type = Type::new(type_token)?;
        let var_name = VarName::new(var_name_token)?;

        Ok(Self {
            r#type,
            var_name,
        })
    }
}
