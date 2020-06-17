use logos::*;
use crate::type_checker::DataTypes;
use crate::type_checker::infer_types;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Logos)]
pub enum Token<'a> {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
    #[regex(r"\.[a-zA-Z]+", |lex| lex.slice())]
    // #[regex(r"\.[a-zA-Z]+\.", |lex| lex.slice())]
    ScriptProperty(&'a str),
    #[regex(r"\$[a-zA-Z_0-9]+", |lex| lex.slice())]
    Variable(&'a str),
    // add all keywords specifically; they will match as scriptproperties if they have a . before them
    #[regex("key", |lex| lex.slice())]
    Keyword(&'a str),
}

pub struct Parsed<'a> {
    pub token: Token<'a>,
    pub pos: std::ops::Range<usize>,
    pub datatype: Option<Vec<DataTypes>>
}
impl<'a> Default for Parsed<'a> {
    fn default() -> Self {
        Parsed {
            token: Token::Error,
            pos: 0..1,
            datatype: None,
        }
    }
    
}

pub fn parse_expression(expression: &str) -> Option<Vec<Parsed>> {
    let mut vec = vec![];
    let mut lex = Token::lexer(expression).spanned();
    while let Some((token, span)) = lex.next() {
        // let datatypes = infer_types(&token);
        vec.push(Parsed {
            token: token,
            pos: span,
            datatype: None,
        })
    }

    if vec.len() > 0 {
        Some(vec)
    } else {
        None
    }
}
