use error::NoneError;
use error::Result;

use crate::stream::peeker::Peeker;

use super::Token;
use super::ComplexBox;
use super::TokenBox;


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

}

const SEMI_SINGLETON: Operator = Operator::Semi;
const COMMA_SINGLETON: Operator = Operator::Comma;
const DOT_SINGLETON: Operator = Operator::Dot;
const OPEN_PAREN_SINGLETON: Operator = Operator::OpenParen;
const CLOSE_PAREN_SINGLETON: Operator = Operator::CloseParen;
const OPEN_BRACE_SINGLETON: Operator = Operator::OpenBrace;
const CLOSE_BRACE_SINGLETON: Operator = Operator::CloseBrace;
const OPEN_BRACKET_SINGLETON: Operator = Operator::OpenBracket;
const CLOSE_BRACKET_SINGLETON: Operator = Operator::CloseBracket;
const AT_SINGLETON: Operator = Operator::At;
const POUND_SINGLETON: Operator = Operator::Pound;
const TILDE_SINGLETON: Operator = Operator::Tilde;
const QUESTION_SINGLETON: Operator = Operator::Question;
const COLON_SINGLETON: Operator = Operator::Colon;
const DOLLAR_SINGLETON: Operator = Operator::Dollar;
const EQ_SINGLETON: Operator = Operator::Eq;
const BANG_SINGLETON: Operator = Operator::Bang;
const LT_SINGLETON: Operator = Operator::Lt;
const GT_SINGLETON: Operator = Operator::Gt;
const MINUS_SINGLETON: Operator = Operator::Minus;
const AND_SINGLETON: Operator = Operator::And;
const OR_SINGLETON: Operator = Operator::Or;
const PLUS_SINGLETON: Operator = Operator::Plus;
const STAR_SINGLETON: Operator = Operator::Star;
const SLASH_SINGLETON: Operator = Operator::Slash;
const CARET_SINGLETON: Operator = Operator::Caret;
const PERCENT_SINGLETON: Operator = Operator::Percent;

const DOUBLE_COLON_SINGLETON: Operator = Operator::DoubleColon;

impl Operator {
    pub fn parse(c: char) -> Option<ComplexBox<dyn Token>> {
        match c {
            ';' => Some(ComplexBox::Ref(&SEMI_SINGLETON)),
            ',' => Some(ComplexBox::Ref(&COMMA_SINGLETON)),
            '.' => Some(ComplexBox::Ref(&DOT_SINGLETON)),
            '(' => Some(ComplexBox::Ref(&OPEN_PAREN_SINGLETON)),
            ')' => Some(ComplexBox::Ref(&CLOSE_PAREN_SINGLETON)),
            '{' => Some(ComplexBox::Ref(&OPEN_BRACE_SINGLETON)),
            '}' => Some(ComplexBox::Ref(&CLOSE_BRACE_SINGLETON)),
            '[' => Some(ComplexBox::Ref(&OPEN_BRACKET_SINGLETON)),
            ']' => Some(ComplexBox::Ref(&CLOSE_BRACKET_SINGLETON)),
            '@' => Some(ComplexBox::Ref(&AT_SINGLETON)),
            '#' => Some(ComplexBox::Ref(&POUND_SINGLETON)),
            '~' => Some(ComplexBox::Ref(&TILDE_SINGLETON)),
            '?' => Some(ComplexBox::Ref(&QUESTION_SINGLETON)),
            ':' => Some(ComplexBox::Ref(&COLON_SINGLETON)),
            '$' => Some(ComplexBox::Ref(&DOLLAR_SINGLETON)),
            '=' => Some(ComplexBox::Ref(&EQ_SINGLETON)),
            '!' => Some(ComplexBox::Ref(&BANG_SINGLETON)),
            '<' => Some(ComplexBox::Ref(&LT_SINGLETON)),
            '>' => Some(ComplexBox::Ref(&GT_SINGLETON)),
            '-' => Some(ComplexBox::Ref(&MINUS_SINGLETON)),
            '&' => Some(ComplexBox::Ref(&AND_SINGLETON)),
            '|' => Some(ComplexBox::Ref(&OR_SINGLETON)),
            '+' => Some(ComplexBox::Ref(&PLUS_SINGLETON)),
            '*' => Some(ComplexBox::Ref(&STAR_SINGLETON)),
            '/' => Some(ComplexBox::Ref(&SLASH_SINGLETON)),
            '^' => Some(ComplexBox::Ref(&CARET_SINGLETON)),
            '%' => Some(ComplexBox::Ref(&PERCENT_SINGLETON)),
            _ => None,
        }
    }

    pub fn complex_parse(peeker: &mut Peeker<TokenBox>) -> Result<TokenBox> {
        if let [a, b] = &peeker.peeks(2)?[..] {
            let a = &*a.downcast::<Operator>()?;
            let b = &*b.downcast::<Operator>()?;
            match (a, b) {
                (Operator::Colon, Operator::Colon) => {
                    peeker.next()?;
                    peeker.next()?;
                    return Ok(ComplexBox::Ref(&DOUBLE_COLON_SINGLETON));
                }
                _ => {}
            }
        };

        Err(NoneError.into())
    }

    pub fn priority(&self) -> isize {
        match self {
            Operator::Eq => 1,
            Operator::Plus | Operator::Minus => 2,
            Operator::Star | Operator::Slash | Operator::Percent => 3,
            _ => -1,
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Operator::Plus | Operator::Minus | Operator::Bang | Operator::Tilde => true,
            _ => false,
        }
    }

}

impl Token for Operator {}
