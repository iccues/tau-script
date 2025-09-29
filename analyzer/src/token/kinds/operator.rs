use std::sync::LazyLock;

use crate::token::Token;
use crate::token::TokenBox;


#[derive(Debug, PartialEq)]
pub enum Operator {
    /// `;`
    Semi,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `@`
    At,
    /// `#`
    Pound,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `!`
    Bang,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `&`
    And,
    /// `|`
    Or,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,

    /// Complex Part
    /// `::`
    DoubleColon,
    /// `==`
    DoubleEq,
    /// `!=`
    NotEq,

}

const SEMI_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Semi));
const COMMA_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Comma));
const DOT_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Dot));
const OPEN_PAREN_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::OpenParen));
const CLOSE_PAREN_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::CloseParen));
const OPEN_BRACE_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::OpenBrace));
const CLOSE_BRACE_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::CloseBrace));
const OPEN_BRACKET_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::OpenBracket));
const CLOSE_BRACKET_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::CloseBracket));
const AT_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::At));
const POUND_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Pound));
const TILDE_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Tilde));
const QUESTION_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Question));
const COLON_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Colon));
const DOLLAR_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Dollar));
const EQ_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Eq));
const BANG_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Bang));
const LT_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Lt));
const GT_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Gt));
const MINUS_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Minus));
const AND_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::And));
const OR_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Or));
const PLUS_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Plus));
const STAR_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Star));
const SLASH_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Slash));
const CARET_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Caret));
const PERCENT_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::Percent));

const DOUBLE_COLON_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::DoubleColon));
const DOUBLE_EQ_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::DoubleEq));
const NOT_EQ_SINGLETON: LazyLock<TokenBox> = LazyLock::new(|| TokenBox::new(Operator::NotEq));

impl Operator {
    pub fn parse_complex(s: &str) -> Option<TokenBox> {
        match s {
            "::" => Some(DOUBLE_COLON_SINGLETON.clone()),
            "==" => Some(DOUBLE_EQ_SINGLETON.clone()),
            "!=" => Some(NOT_EQ_SINGLETON.clone()),
            _ => None,
        }
    }

    pub fn parse(c: char) -> Option<TokenBox> {
        match c {
            ';' => Some(SEMI_SINGLETON.clone()),
            ',' => Some(COMMA_SINGLETON.clone()),
            '.' => Some(DOT_SINGLETON.clone()),
            '(' => Some(OPEN_PAREN_SINGLETON.clone()),
            ')' => Some(CLOSE_PAREN_SINGLETON.clone()),
            '{' => Some(OPEN_BRACE_SINGLETON.clone()),
            '}' => Some(CLOSE_BRACE_SINGLETON.clone()),
            '[' => Some(OPEN_BRACKET_SINGLETON.clone()),
            ']' => Some(CLOSE_BRACKET_SINGLETON.clone()),
            '@' => Some(AT_SINGLETON.clone()),
            '#' => Some(POUND_SINGLETON.clone()),
            '~' => Some(TILDE_SINGLETON.clone()),
            '?' => Some(QUESTION_SINGLETON.clone()),
            ':' => Some(COLON_SINGLETON.clone()),
            '$' => Some(DOLLAR_SINGLETON.clone()),
            '=' => Some(EQ_SINGLETON.clone()),
            '!' => Some(BANG_SINGLETON.clone()),
            '<' => Some(LT_SINGLETON.clone()),
            '>' => Some(GT_SINGLETON.clone()),
            '-' => Some(MINUS_SINGLETON.clone()),
            '&' => Some(AND_SINGLETON.clone()),
            '|' => Some(OR_SINGLETON.clone()),
            '+' => Some(PLUS_SINGLETON.clone()),
            '*' => Some(STAR_SINGLETON.clone()),
            '/' => Some(SLASH_SINGLETON.clone()),
            '^' => Some(CARET_SINGLETON.clone()),
            '%' => Some(PERCENT_SINGLETON.clone()),
            _ => None,
        }
    }

    pub fn priority(&self) -> isize {
        match self {
            Operator::Eq => 1,
            Operator::DoubleEq | Operator::NotEq => 2,
            Operator::Plus | Operator::Minus => 3,
            Operator::Star | Operator::Slash | Operator::Percent => 4,
            _ => -1,
        }
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, Operator::Plus | Operator::Minus | Operator::Bang | Operator::Tilde)
    }
}

impl Token for Operator {}
