//mod error;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

/// This Enum lists the token types that are used by the Forth interpreter
#[derive(Debug)]
pub enum ForthToken {
    Number(i64),
    Command(String),
    Colon(String),
    SemiColon,
    End,
    Error(String),
    Comment(String),
}

struct ForthTokenizer<'a> {
    to_tokenize: &'a str,
    curr: i64,
    next: i64,
    count: i64,
}

// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a> Iterator for ForthTokenizer<'a> {
    type Item = ForthToken;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<ForthToken> {
        if self.count > 5 {
            return None;
        }

        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;
        self.count += 1;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(ForthToken::Number(self.curr))
    }
}

impl<'a> ForthTokenizer<'a> {
    pub fn new(to_tokenize: &'a str) -> ForthTokenizer<'a> {
        ForthTokenizer {
            to_tokenize: to_tokenize,
            curr: 0,
            next: 0,
            count: 0,
        }
    }
}

// This macro lets you statically initialize a hashmap
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
