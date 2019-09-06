/// This Enum lists the token types that are used by the Forth interpreter
#[derive(Debug)]
pub enum ForthToken<'a> {
    Number(i64),
    Command(&'a str),
    StringToken(&'a str),
    Colon,
    SemiColon,
    DropLineComment(&'a str),
    ParenthesizedRemark(&'a str),
}

pub struct ForthTokenizer<'a> {
    to_tokenize: &'a str,
}

impl<'a> ForthTokenizer<'a> {
    pub fn new(to_tokenize: &'a str) -> ForthTokenizer<'a> {
        ForthTokenizer {
            to_tokenize: to_tokenize,
        }
    }
}

impl<'a> IntoIterator for ForthTokenizer<'a> {
    type Item = ForthToken<'a>;
    type IntoIter = ForthTokenizerIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ForthTokenizerIntoIterator {
            forth_tokenizer: self,
        }
    }
}

pub struct ForthTokenizerIntoIterator<'a> {
    forth_tokenizer: ForthTokenizer<'a>,
}

// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for ForthTokenizerIntoIterator<'a> {
    type Item = ForthToken<'a>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<ForthToken<'a>> {
        // We ignore whitespace
        self.forth_tokenizer.to_tokenize = self.forth_tokenizer.to_tokenize.trim_start();

        if let Some(c) = self.forth_tokenizer.to_tokenize.chars().next() {
            return match c {
                '\\' => {
                    let (first, rest) = split_at_newline(self.forth_tokenizer.to_tokenize);
                    self.forth_tokenizer.to_tokenize = rest;
                    Some(ForthToken::DropLineComment(first))
                }
                ':' => {
                    self.forth_tokenizer.to_tokenize = &self.forth_tokenizer.to_tokenize[1..];
                    Some(ForthToken::Colon)
                }
                ';' => {
                    self.forth_tokenizer.to_tokenize = &self.forth_tokenizer.to_tokenize[1..];
                    Some(ForthToken::SemiColon)
                }
                '(' => {
                    let (first, rest) = split_at_token(self.forth_tokenizer.to_tokenize, ')');
                    self.forth_tokenizer.to_tokenize = rest;
                    Some(ForthToken::ParenthesizedRemark(first))
                }
                /* +++ CHECK THIS +++ We haven't implemented strings yet...
                                '"' => {
                                    let (first, rest) = split_at_token(&self.to_tokenize[1..], '"');
                                    self.to_tokenize = rest;
                                    Some(ForthToken::StringToken(first))
                                }
                */
                _ => {
                    let (start, rest) = split_at_ascii_whitespace(self.forth_tokenizer.to_tokenize);
                    self.forth_tokenizer.to_tokenize = rest;
                    Some(ForthToken::Command(start))
                }
            };
        } else {
            return None;
        }
    }
}

fn split_at_newline<'a>(to_split: &'a str) -> (&'a str, &'a str) {
    let mut line_iterator = to_split.splitn(2, &['\n', '\r'][..]);
    if let Some(first) = line_iterator.next() {
        if let Some(rest) = line_iterator.next() {
            return match rest.chars().next().unwrap() {
                '\n' => (first, &rest[1..]),
                _ => (first, rest),
            };
        } else {
            return (first, "");
        }
    } else {
        return ("", "");
    }
}

fn split_at_ascii_whitespace<'a>(to_split: &'a str) -> (&'a str, &'a str) {
    let mut line_iterator = to_split.splitn(2, |c: char| c.is_ascii_whitespace());
    if let Some(first) = line_iterator.next() {
        if let Some(rest) = line_iterator.next() {
            return match rest.chars().next().unwrap() {
                '\n' => (first, &rest[1..]),
                _ => (first, rest),
            };
        } else {
            return (first, "");
        }
    } else {
        return ("", "");
    }
}

fn split_at_token<'a>(to_split: &'a str, token: char) -> (&'a str, &'a str) {
    let mut line_iterator = to_split.splitn(2, token);
    if let Some(first) = line_iterator.next() {
        if let Some(rest) = line_iterator.next() {
            return match rest.chars().next().unwrap() {
                '\n' => (first, &rest[1..]),
                _ => (first, rest),
            };
        } else {
            return (first, "");
        }
    } else {
        return ("", "");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_newline_1() {
        assert_eq!(split_at_newline(""), ("", ""));
    }

    #[test]
    fn test_split_at_newline_2() {
        assert_eq!(split_at_newline("abc"), ("abc", ""));
    }

    #[test]
    fn test_split_at_newline_3() {
        assert_eq!(split_at_newline("abc\r\ndef"), ("abc", "def"));
    }

    #[test]
    fn test_split_at_newline_4() {
        assert_eq!(split_at_newline("abc\ndef"), ("abc", "def"));
        assert_eq!(split_at_newline(""), ("", ""));
    }
    #[test]
    fn test_split_at_newline_5() {
        assert_eq!(
            split_at_newline("abc\r\ndef\r\nghi\r\njkl"),
            ("abc", "def\r\nghi\r\njkl")
        );
    }
    #[test]
    fn test_split_at_newline_6() {
        assert_eq!(
            split_at_newline("abc\ndef\nghi\njkl"),
            ("abc", "def\nghi\njkl")
        );
        assert_eq!(split_at_newline(""), ("", ""));
    }
}
