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
const DOUBLE_EQ_SINGLETON: Operator = Operator::DoubleEq;
const NOT_EQ_SINGLETON: Operator = Operator::NotEq;

impl Operator {
    pub fn parse_complex(s: &str) -> Option<TokenBox> {
        match s {
            "::" => Some(TokenBox::Ref(&DOUBLE_COLON_SINGLETON)),
            "==" => Some(TokenBox::Ref(&DOUBLE_EQ_SINGLETON)),
            "!=" => Some(TokenBox::Ref(&NOT_EQ_SINGLETON)),
            _ => None,
        }
    }

    pub fn parse(c: char) -> Option<TokenBox> {
        match c {
            ';' => Some(TokenBox::Ref(&SEMI_SINGLETON)),
            ',' => Some(TokenBox::Ref(&COMMA_SINGLETON)),
            '.' => Some(TokenBox::Ref(&DOT_SINGLETON)),
            '(' => Some(TokenBox::Ref(&OPEN_PAREN_SINGLETON)),
            ')' => Some(TokenBox::Ref(&CLOSE_PAREN_SINGLETON)),
            '{' => Some(TokenBox::Ref(&OPEN_BRACE_SINGLETON)),
            '}' => Some(TokenBox::Ref(&CLOSE_BRACE_SINGLETON)),
            '[' => Some(TokenBox::Ref(&OPEN_BRACKET_SINGLETON)),
            ']' => Some(TokenBox::Ref(&CLOSE_BRACKET_SINGLETON)),
            '@' => Some(TokenBox::Ref(&AT_SINGLETON)),
            '#' => Some(TokenBox::Ref(&POUND_SINGLETON)),
            '~' => Some(TokenBox::Ref(&TILDE_SINGLETON)),
            '?' => Some(TokenBox::Ref(&QUESTION_SINGLETON)),
            ':' => Some(TokenBox::Ref(&COLON_SINGLETON)),
            '$' => Some(TokenBox::Ref(&DOLLAR_SINGLETON)),
            '=' => Some(TokenBox::Ref(&EQ_SINGLETON)),
            '!' => Some(TokenBox::Ref(&BANG_SINGLETON)),
            '<' => Some(TokenBox::Ref(&LT_SINGLETON)),
            '>' => Some(TokenBox::Ref(&GT_SINGLETON)),
            '-' => Some(TokenBox::Ref(&MINUS_SINGLETON)),
            '&' => Some(TokenBox::Ref(&AND_SINGLETON)),
            '|' => Some(TokenBox::Ref(&OR_SINGLETON)),
            '+' => Some(TokenBox::Ref(&PLUS_SINGLETON)),
            '*' => Some(TokenBox::Ref(&STAR_SINGLETON)),
            '/' => Some(TokenBox::Ref(&SLASH_SINGLETON)),
            '^' => Some(TokenBox::Ref(&CARET_SINGLETON)),
            '%' => Some(TokenBox::Ref(&PERCENT_SINGLETON)),
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
