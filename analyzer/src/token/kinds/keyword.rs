use std::sync::LazyLock;

use crate::error::AnalyzerError;
use crate::error::Result;
use crate::token::identifier::Identifier;
use crate::token::Token;
use crate::token::TokenBox;

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

    True,
    False,
}

const LET_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Let));
const MOD_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Mod));
const DEF_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Def));
const TYPE_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Type));
const VAR_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Var));
const FUNC_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Func));
const IF_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::If));
const ELSE_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Else));
const WHILE_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::While));
const SELF_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::Self_));

const TRUE_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::True));
const FALSE_KEYWORD: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Keyword::False));

impl Keyword {
    pub fn parse(token: TokenBox) -> Result<TokenBox> {
        let identifier = token.downcast::<Identifier>()?;
        match &identifier.name()[..] {
            "let" => Ok(LET_KEYWORD.clone()),
            "mod" => Ok(MOD_KEYWORD.clone()),
            "def" => Ok(DEF_KEYWORD.clone()),
            "type" => Ok(TYPE_KEYWORD.clone()),
            "var" => Ok(VAR_KEYWORD.clone()),
            "func" => Ok(FUNC_KEYWORD.clone()),
            "if" => Ok(IF_KEYWORD.clone()),
            "else" => Ok(ELSE_KEYWORD.clone()),
            "while" => Ok(WHILE_KEYWORD.clone()),
            "self" => Ok(SELF_KEYWORD.clone()),

            "true" => Ok(TRUE_KEYWORD.clone()),
            "false" => Ok(FALSE_KEYWORD.clone()),
            _ => Err(AnalyzerError::None),
        }
    }
}

impl Token for Keyword {}
