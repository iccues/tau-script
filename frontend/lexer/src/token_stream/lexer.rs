use frontend_library::error::{FrontendError, FrontendResult as Result};
use frontend_library::stream::Position;

use crate::char_stream::EOF_CHAR;
use frontend_library::stream::{peeker::Peeker, Stream};
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

impl Stream for Lexer {
    type Item = TokenBox;

    fn next(&mut self) -> Result<Self::Item> {
        self.next_token()
    }

    fn last_position(&self) -> Position {
        self.char_peeker.last_position()
    }
    fn next_position(&self) -> Position {
        self.char_peeker.next_position()
    }
}

impl Lexer {
    pub fn new(char_peeker: Peeker<char>) -> Self {
        Self {
            char_peeker,
        }
    }

    pub fn next_token(&mut self) -> Result<TokenBox> {
        self.skip_whitespace()?;

        try_parse!(self.parse_comment());
        try_parse!(self.parse_ident());
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
        match &self.char_peeker.peek_str(2)?[..] {
            "//" => {
                let mut text = String::new();

                self.char_peeker.next()?;
                self.char_peeker.next()?;

                let mut c = self.char_peeker.peek()?;
                while c != '\n' && c != EOF_CHAR {
                    text.push(self.char_peeker.next()?);
                    c = self.char_peeker.peek()?;
                }
                Ok(Comment::new(Some(text)))

            }
            "/*" => {
                let mut text = String::new();

                self.char_peeker.next()?;
                self.char_peeker.next()?;
    
                let mut s = self.char_peeker.peek_str(2)?;
                while s != "*/" {
                    text.push(self.char_peeker.next()?);
                    s = self.char_peeker.peek_str(2)?;
                }
    
                self.char_peeker.next()?;
                self.char_peeker.next()?;
    
                Ok(Comment::new(Some(text)))
    
            }
            _ => Err(FrontendError::None),
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
                Ok(Float::new(num))
            } else {
                Ok(Integer::new(num))
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
            Ok(StringToken::new(string))
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
