use frontend_library::error::{FrontendError, FrontendResult as Result};
use frontend_library::token::keyword::Keyword;

use crate::char_stream::EOF_CHAR;
use frontend_library::stream::peeker::Peeker;
use frontend_library::token::comment::Comment;
use frontend_library::token::identifier::Identifier;
use frontend_library::token::number::{Float, Integer};
use frontend_library::token::operator::Operator;
use frontend_library::token::singleton_token::EofToken;
use frontend_library::token::string::StringToken;
use frontend_library::token::TokenBox;
use frontend_library::try_parse;


pub struct Lexer {
    char_peeker: Peeker<char>,
}


impl Lexer {
    pub fn new(char_peeker: Peeker<char>) -> Self {
        Self {
            char_peeker,
        }
    }

    pub fn next(&mut self) -> Result<TokenBox> {
        self.skip_whitespace()?;

        try_parse!(self.parse_comment());
        try_parse!(self.parse_ident_and_keyword());
        try_parse!(self.parse_number());
        try_parse!(self.parse_string());
        try_parse!(self.parse_operator());
        try_parse!(self.parse_eof());

        Err(FrontendError::UnknownToken)
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            if self.char_peeker.peek()?.is_whitespace() {
                self.char_peeker.next()?;
            }
            else {
                break;
            }
        }
        Ok(())
    }

    fn parse_comment(&mut self) -> Result<TokenBox> {
        try_parse!(self.parse_comment_short());
        try_parse!(self.parse_comment_long());
        Err(FrontendError::None)
    }
    fn parse_comment_short(&mut self) -> Result<TokenBox> {
        self.char_peeker.eat_str("//")?;
        let mut text = String::new();
        let mut c = self.char_peeker.peek()?;
        while c != '\n' && c != EOF_CHAR {
            text.push(self.char_peeker.next()?);
            c = self.char_peeker.peek()?;
        }
        Ok(Comment::new(Some(text)))
    }
    fn parse_comment_long(&mut self) -> Result<TokenBox> {
        self.char_peeker.eat_str("/*")?;
        let mut text = String::new();
        loop {
            match self.char_peeker.eat_str("*/") {
                Ok(()) => return Ok(Comment::new(Some(text))),
                Err(FrontendError::None) => {
                    let c = self.char_peeker.next()?;
                    text.push(c);
                },
                Err(e) => return Err(e),
            }
        }
    }

    // TODO
    fn parse_ident_and_keyword(&mut self) -> Result<TokenBox> {
        let ident = self.parse_ident()?;
        if let Ok(keyword) = Keyword::parse(ident.clone()) {
            Ok(keyword)
        } else {
            Ok(ident)
        }
    }
    fn parse_ident(&mut self) -> Result<TokenBox> {
        let mut c = self.char_peeker.peek()?;
        
        if c.is_alphabetic() || c == '_' {
            let mut name = String::new();
            while c.is_alphanumeric() || c == '_' {
                name.push(self.char_peeker.next()?);
                c = self.char_peeker.peek()?;
            }
            Ok(Identifier::new(name))
        }

        else {
            Err(FrontendError::None)
        }
    }

    fn parse_number(&mut self) -> Result<TokenBox> {
        let mut c = self.char_peeker.peek()?;

        if c.is_numeric() {
            let mut num = String::new();
            while c.is_numeric() {
                num.push(self.char_peeker.next()?);
                c = self.char_peeker.peek()?;
            }
            if c == '.' {
                num.push(self.char_peeker.next()?);
                c = self.char_peeker.peek()?;
                while c.is_numeric() {
                    num.push(self.char_peeker.next()?);
                    c = self.char_peeker.peek()?;
                }
                Float::new(num)
            } else {
                Integer::new(num)
            }
        }

        else {
            Err(FrontendError::None)
        }
    }

    fn parse_string(&mut self) -> Result<TokenBox> {
        let mut c = self.char_peeker.peek()?;

        if c == '"' {
            let mut string = String::new();
            string.push(self.char_peeker.next()?);
            c = self.char_peeker.peek()?;
            while c != '"' {
                string.push(self.char_peeker.next()?);
                c = self.char_peeker.peek()?;
            }
            string.push(self.char_peeker.next()?);
            StringToken::new(string)
        } else {
            Err(FrontendError::None)
        }
    }

    fn parse_operator(&mut self) -> Result<TokenBox> {
        let s = self.char_peeker.peek_str(2)?;
        if let Some(token_type) = Operator::parse_complex(&s) {
            self.char_peeker.next()?;
            self.char_peeker.next()?;
            return Ok(token_type);
        }
        let c = self.char_peeker.peek()?;
        if let Some(token_type) = Operator::parse(c) {
            self.char_peeker.next()?;
            Ok(token_type)
        }
        else {
            Err(FrontendError::None)
        }
    }

    fn parse_eof(&mut self) -> Result<TokenBox> {
        if self.char_peeker.peek()? == EOF_CHAR {
            Ok(EofToken::new())
        } else {
            Err(FrontendError::None)
        }
    }
}
