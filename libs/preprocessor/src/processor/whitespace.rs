use peekmore::PeekMoreIterator;

use crate::{token::Token, Error};

use super::Processor;

impl Processor {
    /// Skip whitespace
    /// The stream is left after the whitespace
    pub(crate) fn skip_whitespace(
        &mut self,
        stream: &mut PeekMoreIterator<impl Iterator<Item = Token>>,
        mut buffer: Option<&mut Vec<Token>>,
    ) {
        while let Some(token) = stream.peek() {
            if token.symbol().is_whitespace() {
                let token = stream.next().expect("was peeked");
                if let Some(inner) = buffer {
                    self.output(token, inner);
                    buffer = Some(inner);
                }
            } else {
                break;
            }
        }
    }

    /// Skip to the next newline
    /// The stream is left after the newline
    /// End of input will not cause an error
    pub(crate) fn skip_to_after_newline(
        &mut self,
        stream: &mut PeekMoreIterator<impl Iterator<Item = Token>>,
        mut buffer: Option<&mut Vec<Token>>,
    ) {
        while stream.peek().is_some() {
            let token = stream.next().expect("was peeked");
            let symbol = token.symbol().clone();
            if let Some(inner) = buffer {
                self.output(token, inner);
                buffer = Some(inner);
            }
            if symbol.is_newline() {
                break;
            }
        }
    }

    /// Expect no content until the next newline
    /// Whitespace is allowed, but nothing else
    /// The stream is left after the newline
    pub(crate) fn expect_nothing_to_newline(
        &mut self,
        stream: &mut PeekMoreIterator<impl Iterator<Item = Token>>,
    ) -> Result<(), Error> {
        for token in stream.by_ref() {
            if token.symbol().is_newline() {
                break;
            }
            if !token.symbol().is_whitespace() {
                panic!("unexpected token: {:?}", token);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        processor::{tests, Processor},
        symbol::Symbol,
    };

    #[test]
    fn test_skip_whitespace_space() {
        let mut stream = tests::setup("  a");
        let mut processor = Processor::default();
        processor.skip_whitespace(&mut stream, None);
        assert_eq!(
            *stream.next().unwrap().symbol(),
            Symbol::Word("a".to_string())
        );
    }

    #[test]
    fn test_skip_whitespace_tab() {
        let mut stream = tests::setup("\ta");
        let mut processor = Processor::default();
        processor.skip_whitespace(&mut stream, None);
        assert_eq!(
            *stream.next().unwrap().symbol(),
            Symbol::Word("a".to_string())
        );
    }

    #[test]
    fn test_skip_whitespace_newline() {
        let mut stream = tests::setup("\na");
        let mut processor = Processor::default();
        processor.skip_whitespace(&mut stream, None);
        assert_eq!(*stream.next().unwrap().symbol(), Symbol::Newline);
    }

    #[test]
    fn test_skip_whitespace_eoi() {
        let mut stream = tests::setup("");
        let mut processor = Processor::default();
        processor.skip_whitespace(&mut stream, None);
        assert_eq!(*stream.next().unwrap().symbol(), Symbol::Eoi);
    }

    #[test]
    fn test_skip_to_after_newline() {
        let mut stream = tests::setup("a\nb");
        let mut processor = Processor::default();
        processor.skip_to_after_newline(&mut stream, None);
        assert_eq!(
            *stream.next().unwrap().symbol(),
            Symbol::Word("b".to_string())
        );
    }

    #[test]
    fn test_expect_nothing_to_newline_whitespace() {
        let mut stream = tests::setup("  \nb");
        let mut processor = Processor::default();
        processor.expect_nothing_to_newline(&mut stream).unwrap();
    }
}
