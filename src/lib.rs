//mod error;

//use std::collections::HashMap;

/// This Enum lists the token types that are used by the Forth interpreter
#[derive(Debug)]
pub enum ForthToken<'a> {
    Number(i64),
    Command(&'a str),
    Colon,
    SemiColon,
    DeleteMeNOP,
    Error(&'a str),
    Comment(&'a str),
    DeleteMeLocalDef(&'a str),
}

pub struct ForthTokenizer<'a> {
    to_tokenize: &'a str,
    curr: usize,
    count: i32,
}

// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for ForthTokenizer<'a> {
    type Item = ForthToken<'a>;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<ForthToken<'a>> {
        let check_this = &self.to_tokenize[self.curr..];

        if let Some(c) = check_this.chars().next() {
            return match c {
                '\\' => {
                    let (first, rest) = split_at_newline(self.to_tokenize);
                    self.to_tokenize = rest;
                    Some(ForthToken::Comment(first))
                }
                ':' => {
                    self.to_tokenize = &self.to_tokenize[1..];
                    Some(ForthToken::Colon)
                }
                ';' => {
                    self.to_tokenize = &self.to_tokenize[1..];
                    Some(ForthToken::SemiColon)
                }
                '(' => {
                    let (first, rest) = split_at_token(self.to_tokenize, ')');
                    self.to_tokenize = rest;
                    Some(ForthToken::DeleteMeLocalDef(first))
                }
                _ => {
                    let to_tokenize = self.to_tokenize.trim_start();
                    let (start, rest) = split_at_ascii_whitespace(to_tokenize);
                    self.to_tokenize = rest;
                    Some(ForthToken::Command(start))
                }
            };
        } else {
            return None;
        }
    }
}

impl<'a> ForthTokenizer<'a> {
    pub fn new(to_tokenize: &'a str) -> ForthTokenizer<'a> {
        ForthTokenizer {
            to_tokenize: to_tokenize,
            curr: 0,
            count: 0,
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
    let mut line_iterator = to_split.splitn(2, |c:char| c.is_ascii_whitespace());
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
