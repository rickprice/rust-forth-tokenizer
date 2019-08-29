mod error;

use logos::Logos;
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

#[derive(Logos, Debug, PartialEq)]
pub enum ForthLexerToken {
    // Logos requires that we define two default variants,
    // one for end of input source,
    #[end]
    End,

    // ...and one for errors. Those can be named anything
    // you wish as long as the attributes are there.
    #[error]
    Error,

    #[regex = "\\\\.*\n"]
    SingleLineComment,

    #[regex = "\\(.*\\)"]
    Comment,

    #[regex = "\\{.*\\}"]
    LocalVariableDefinition,

    #[token = ":"]
    Colon,

    #[token = ";"]
    SemiColon,

    #[regex = "[a-zA-Z0-<>9|+-/*_{}#$=!@\"\\[\\]()?'&%~]+"]
    Command,

    #[regex = "[0-9]+"]
    Number,
}

// This macro lets you statically initialize a hashmap
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}
