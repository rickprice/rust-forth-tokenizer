/// This is how to use the Forth tokenizer library
/// ```
/// use rust_forth_tokenizer::ForthToken;
/// use rust_forth_tokenizer::ForthTokenizer;
///
///     let tokenizer = ForthTokenizer::new("word : wordname 1 2 3 ; definition");
///     // The code also supports the regular for loop iterator syntax
///     let collected: Vec<_> = tokenizer.into_iter().collect();
///     assert_eq!(
///            &collected,
///            &vec![
///                ForthToken::Command("word"),
///                ForthToken::Colon,
///                ForthToken::Command("wordname"),
///                ForthToken::Number(1),
///                ForthToken::Number(2),
///                ForthToken::Number(3),
///                ForthToken::SemiColon,
///                ForthToken::Command("definition"),
///            ]
///        );
/// ```

/// This Enum lists the token types that are used by the Forth interpreter
#[derive(Debug, PartialEq)]
pub enum ForthToken<'a> {
    Number(i64),
    Command(&'a str),
    // Command, string
    StringCommand(&'a str, &'a str),
    Colon,
    SemiColon,
    DropLineComment(&'a str),
    ParenthesizedRemark(&'a str),
}

/// This is the ForthTokenizer, it is the actual tokenizer
pub struct ForthTokenizer<'a> {
    to_tokenize: &'a str,
}

impl<'a> ForthTokenizer<'a> {
    pub fn new(to_tokenize: &'a str) -> ForthTokenizer<'a> {
        ForthTokenizer { to_tokenize }
    }
}

impl<'a> IntoIterator for ForthTokenizer<'a> {
    type Item = ForthToken<'a>;
    type IntoIter = ForthTokenizerIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ForthTokenizerIntoIterator {
            to_tokenize: self.to_tokenize,
        }
    }
}

pub struct ForthTokenizerIntoIterator<'a> {
    to_tokenize: &'a str,
}

// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for ForthTokenizerIntoIterator<'a> {
    type Item = ForthToken<'a>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<ForthToken<'a>> {
        // We ignore whitespace
        self.to_tokenize = self.to_tokenize.trim_start();

        if let Some(c) = self.to_tokenize.chars().next() {
            match c {
                '\\' => {
                    let (first, rest) = split_at_newline(self.to_tokenize);
                    self.to_tokenize = rest;
                    Some(ForthToken::DropLineComment(first))
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
                    Some(ForthToken::ParenthesizedRemark(first))
                }
                _ => {
                    let (start, rest) = split_at_ascii_whitespace(self.to_tokenize);
                    self.to_tokenize = rest;

                    if start.ends_with('"') {
                        let (newstart, newrest) = split_at_token(rest, '"');
                        self.to_tokenize = newrest;

                        return Some(ForthToken::StringCommand(&start, newstart));
                    }
                    // Determine if its a number or a command
                    match start.parse::<i64>() {
                        // We found a number, then return it as a number token
                        Ok(n) => Some(ForthToken::Number(n)),
                        // Wasn't a number, treat it as a *word*
                        Err(_) => Some(ForthToken::Command(start)),
                    }
                }
            }
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a ForthTokenizer<'a> {
    type Item = ForthToken<'a>;
    type IntoIter = ForthTokenizerIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ForthTokenizerIntoIterator {
            to_tokenize: self.to_tokenize,
        }
    }
}

fn split_at_newline(to_split: &str) -> (&str, &str) {
    let mut line_iterator = to_split.splitn(2, &['\n', '\r'][..]);
    if let Some(first) = line_iterator.next() {
        if let Some(rest) = line_iterator.next() {
            match rest.chars().next().unwrap() {
                '\n' => (first, &rest[1..]),
                _ => (first, rest),
            }
        } else {
            (first, "")
        }
    } else {
        ("", "")
    }
}

fn split_at_ascii_whitespace(to_split: &str) -> (&str, &str) {
    let mut line_iterator = to_split.splitn(2, |c: char| c.is_ascii_whitespace());
    if let Some(first) = line_iterator.next() {
        if let Some(rest) = line_iterator.next() {
            match rest.chars().next().unwrap() {
                '\n' => (first, &rest[1..]),
                _ => (first, rest),
            }
        } else {
            (first, "")
        }
    } else {
        ("", "")
    }
}

fn split_at_token(to_split: &str, token: char) -> (&str, &str) {
    let mut line_iterator = to_split.splitn(2, token);
    if let Some(first) = line_iterator.next() {
        if let Some(rest) = line_iterator.next() {
            match rest.chars().next().unwrap() {
                '\n' => (first, &rest[1..]),
                _ => (first, rest),
            }
        } else {
            (first, "")
        }
    } else {
        ("", "")
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

    #[test]
    fn test_number_1() {
        let tokenizer = ForthTokenizer::new("1 these 2 are 3 words 4");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Command("these"),
                ForthToken::Number(2),
                ForthToken::Command("are"),
                ForthToken::Number(3),
                ForthToken::Command("words"),
                ForthToken::Number(4),
            ]
        );
    }

    #[test]
    fn test_command_1() {
        let tokenizer = ForthTokenizer::new("these are #words 1 with 2 numbers");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Command("these"),
                ForthToken::Command("are"),
                ForthToken::Command("#words"),
                ForthToken::Number(1),
                ForthToken::Command("with"),
                ForthToken::Number(2),
                ForthToken::Command("numbers"),
            ]
        );
    }

    #[test]
    fn test_colon_1() {
        let tokenizer = ForthTokenizer::new("word : wordname 1 2 3 ; definition");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Command("word"),
                ForthToken::Colon,
                ForthToken::Command("wordname"),
                ForthToken::Number(1),
                ForthToken::Number(2),
                ForthToken::Number(3),
                ForthToken::SemiColon,
                ForthToken::Command("definition"),
            ]
        );
    }

    #[test]
    fn test_semicolon_1() {
        let tokenizer = ForthTokenizer::new("word : wordname 1 $whatever 3 ; definition");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Command("word"),
                ForthToken::Colon,
                ForthToken::Command("wordname"),
                ForthToken::Number(1),
                ForthToken::Command("$whatever"),
                ForthToken::Number(3),
                ForthToken::SemiColon,
                ForthToken::Command("definition"),
            ]
        );
    }

    #[test]
    fn test_stringcommand_1() {
        let tokenizer = ForthTokenizer::new("1 2 \" This is a string\" 3 4");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Number(2),
                ForthToken::StringCommand("\"", "This is a string"),
                ForthToken::Number(3),
                ForthToken::Number(4),
            ]
        );
    }

    #[test]
    fn test_stringcommand_2() {
        let tokenizer = ForthTokenizer::new("1 2 .s\" This is a string\" 3 4");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Number(2),
                ForthToken::StringCommand(".s\"", "This is a string"),
                ForthToken::Number(3),
                ForthToken::Number(4),
            ]
        );
    }

    #[test]
    fn test_droplinecomment_1() {
        // Forgot the space after the 2, this will come out totally differently than a comment
        let tokenizer = ForthTokenizer::new("1 2\\ This is a dropline comment\n\r1 3\r\n4");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Command("2\\"),
                ForthToken::Command("This"),
                ForthToken::Command("is"),
                ForthToken::Command("a"),
                ForthToken::Command("dropline"),
                ForthToken::Command("comment"),
                ForthToken::Number(1),
                ForthToken::Number(3),
                ForthToken::Number(4),
            ]
        );
    }

    #[test]
    fn test_droplinecomment_2() {
        let tokenizer = ForthTokenizer::new("1 2 \\ This is a dropline comment\n\r1 3\r\n4");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Number(2),
                ForthToken::DropLineComment("\\ This is a dropline comment"),
                ForthToken::Number(1),
                ForthToken::Number(3),
                ForthToken::Number(4),
            ]
        );
    }

    #[test]
    fn test_parenthesized_remark_1() {
        // This isn't maybe intuitive, but we lose the trailing ) because its a delimiter... No easy way to change that that I know of
        let tokenizer = ForthTokenizer::new(
            "1 2 \\ This is a dropline comment ( This is not a parenthesized remark )\n\r1 ( This is in fact a parenthesized remark )3\r\n4",
        );
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Number(2),
                ForthToken::DropLineComment(
                    "\\ This is a dropline comment ( This is not a parenthesized remark )"
                ),
                ForthToken::Number(1),
                ForthToken::ParenthesizedRemark("( This is in fact a parenthesized remark "),
                ForthToken::Number(3),
                ForthToken::Number(4),
            ]
        );
    }

    #[test]
    fn test_bug_1() {
        let tokenizer = ForthTokenizer::new("1 1 1\n2 2 2\n3 3 3");
        let collected: Vec<_> = tokenizer.into_iter().collect();
        assert_eq!(
            &collected,
            &vec![
                ForthToken::Number(1),
                ForthToken::Number(1),
                ForthToken::Number(1),
                ForthToken::Number(2),
                ForthToken::Number(2),
                ForthToken::Number(2),
                ForthToken::Number(3),
                ForthToken::Number(3),
                ForthToken::Number(3)
            ]
        );
    }
}
