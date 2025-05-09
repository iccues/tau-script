use error::NoneError;
use error::Result;

use crate::stream::peeker::Peeker;

use super::identifier::Identifier;
use super::Token;
use super::ComplexBox;
use super::TokenBox;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    Mod,
    Def,
    Type,
    Var,
    Func,
    If,
    Else,
    While,
    Self_,
}

const LET_KEYWORD: Keyword = Keyword::Let;
const MOD_KEYWORD: Keyword = Keyword::Mod;
const DEF_KEYWORD: Keyword = Keyword::Def;
const TYPE_KEYWORD: Keyword = Keyword::Type;
const VAR_KEYWORD: Keyword = Keyword::Var;
const FUNC_KEYWORD: Keyword = Keyword::Func;
const IF_KEYWORD: Keyword = Keyword::If;
const ELSE_KEYWORD: Keyword = Keyword::Else;
const WHILE_KEYWORD: Keyword = Keyword::While;
const SELF_KEYWORD: Keyword = Keyword::Self_;

impl Keyword {
    pub fn parse(cursor: &mut Peeker<TokenBox>) -> Result<TokenBox> {
        let identifier = cursor.peek()?.downcast::<Identifier>()?;
        let ret: Result<TokenBox> = match &identifier.name()[..] {
            "let" => Ok(ComplexBox::Ref(&LET_KEYWORD)),
            "mod" => Ok(ComplexBox::Ref(&MOD_KEYWORD)),
            "def" => Ok(ComplexBox::Ref(&DEF_KEYWORD)),
            "type" => Ok(ComplexBox::Ref(&TYPE_KEYWORD)),
            "var" => Ok(ComplexBox::Ref(&VAR_KEYWORD)),
            "expr" => Ok(ComplexBox::Ref(&FUNC_KEYWORD)),
            "if" => Ok(ComplexBox::Ref(&IF_KEYWORD)),
            "else" => Ok(ComplexBox::Ref(&ELSE_KEYWORD)),
            "while" => Ok(ComplexBox::Ref(&WHILE_KEYWORD)),
            "self" => Ok(ComplexBox::Ref(&SELF_KEYWORD)),
            _ => return Err(NoneError.into()),
        };
        cursor.next()?;
        ret
    }
}

impl Token for Keyword {}