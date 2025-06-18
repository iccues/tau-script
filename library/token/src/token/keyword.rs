use error::NoneError;
use error::Result;

use crate::identifier::Identifier;
use crate::Token;
use crate::TokenBox;

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
    pub fn parse(token: TokenBox) -> Result<TokenBox> {
        let identifier = token.downcast::<Identifier>()?;
        match &identifier.name()[..] {
            "let" => Ok(TokenBox::Ref(&LET_KEYWORD)),
            "mod" => Ok(TokenBox::Ref(&MOD_KEYWORD)),
            "def" => Ok(TokenBox::Ref(&DEF_KEYWORD)),
            "type" => Ok(TokenBox::Ref(&TYPE_KEYWORD)),
            "var" => Ok(TokenBox::Ref(&VAR_KEYWORD)),
            "expr" => Ok(TokenBox::Ref(&FUNC_KEYWORD)),
            "if" => Ok(TokenBox::Ref(&IF_KEYWORD)),
            "else" => Ok(TokenBox::Ref(&ELSE_KEYWORD)),
            "while" => Ok(TokenBox::Ref(&WHILE_KEYWORD)),
            "self" => Ok(TokenBox::Ref(&SELF_KEYWORD)),
            _ => Err(NoneError.into()),
        }
    }
}

impl Token for Keyword {}
