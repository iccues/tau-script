use crate::{error::{FrontendError, FrontendResult}, source::Source};

#[derive(Clone, Copy)]
pub struct Cursor<'src> {
    source: &'src dyn Source,
    index: usize,
}

impl Cursor<'_> {
    pub fn new<'src>(source: &'src dyn Source) -> Cursor<'src> {
        Cursor { source, index: 0 }
    }

    pub fn next(&mut self) -> FrontendResult<char> {
        let (ch, next_index) = self.source.next(self.index)?;
        self.index = next_index;
        Ok(ch)
    }

    pub fn peek(&self) -> FrontendResult<char> {
        self.source.get_char(self.index)
    }

    pub fn eat_str(&mut self, s: &str) -> FrontendResult<()> {
        let mut cursor = *self;
        for ch in s.chars() {
            if cursor.next()? != ch {
                return Err(FrontendError::None);
            }
        }
        *self = cursor;
        Ok(())
    }

    pub fn peek_str(&self, n: usize) -> FrontendResult<String> {
        let mut cursor = *self;
        let mut result = String::new();
        for _ in 0..n {
            result.push(cursor.next()?);
        }
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use crate::source::EOF_CHAR;
    use crate::source::repl_source::ReplSource;

    use super::*;

    #[test]
    fn test_str_source() {
        let src = String::from("hello, 世界!");
        let mut cursor = Cursor::new(&src);

        assert_eq!(cursor.peek().unwrap(), 'h');
        assert_eq!(cursor.next().unwrap(), 'h');
        assert_eq!(cursor.peek().unwrap(), 'e');
        assert_eq!(cursor.next().unwrap(), 'e');
        assert_eq!(cursor.next().unwrap(), 'l');
        assert_eq!(cursor.next().unwrap(), 'l');
        assert_eq!(cursor.next().unwrap(), 'o');
        assert_eq!(cursor.peek().unwrap(), ',');
        assert!(cursor.eat_str(", ").is_ok());
        assert!(cursor.eat_str("!").is_err());
        assert_eq!(cursor.peek().unwrap(), '世');
        assert!(cursor.eat_str("世界!").is_ok());
        assert_eq!(cursor.peek().unwrap(), EOF_CHAR);
        assert_eq!(cursor.next().unwrap(), EOF_CHAR);
    }

    #[test]
    fn test_repl_source() {
        let reader = "hello, 世界!".as_bytes();
        let src = ReplSource::new(reader);
        let mut cursor = Cursor::new(&src);

        assert_eq!(cursor.peek().unwrap(), 'h');
        assert_eq!(cursor.next().unwrap(), 'h');
        assert_eq!(cursor.peek().unwrap(), 'e');
        assert_eq!(cursor.next().unwrap(), 'e');
        assert_eq!(cursor.next().unwrap(), 'l');
        assert_eq!(cursor.next().unwrap(), 'l');
        assert_eq!(cursor.next().unwrap(), 'o');
        assert_eq!(cursor.peek().unwrap(), ',');
        assert!(cursor.eat_str(", ").is_ok());
        assert!(cursor.eat_str("!").is_err());
        assert_eq!(cursor.peek().unwrap(), '世');
        assert!(cursor.eat_str("世界!").is_ok());
    }
}
