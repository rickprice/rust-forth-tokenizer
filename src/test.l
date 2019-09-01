pub struct SpaceCounter {
    count: usize,
}

impl SpaceCounter {
    pub fn new() -> SpaceCounter {
        SpaceCounter {
            count: 0,
        }
    }
    pub fn increment_space(&mut self) {
        self.count += 1;
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

/// This Enum lists the token types that are used by the Forth interpreter
#[derive(Debug)]
pub enum ForthToken {
    Comment(String),
    Number(i64),
    ForthWord(String),
    Colon,
    SemiColon,
    End,
    Error(String),
    DeleteMe(String),
    DeleteMeLocalDef(String),
}

%%
%class Lexer
%result_type ForthToken
%field SpaceCounter space_counter

\\.*$                  return Ok(ForthToken::Comment(self.yytext().clone()));
;.*$                   return Ok(ForthToken::Comment(self.yytext().clone()));
{.*}                   return Ok(ForthToken::Comment(self.yytext().clone()));
\(.*\)                 return Ok(ForthToken::DeleteMeLocalDef(self.yytext().clone()));
\n                     { self.space_counter.increment_space(); }
\t                     { self.space_counter.increment_space(); }
\r                     { self.space_counter.increment_space(); }
" "                    { self.space_counter.increment_space(); }
[a-zA-Z_0-9{}+\-\.#@$?]+     return Ok(ForthToken::ForthWord(self.yytext()));
[0-9-]+                  return Ok(ForthToken::Number(self.yytext().parse::<i64>().expect("Unexpected failure converting number")));
:                       return Ok(ForthToken::Colon);
;                       return Ok(ForthToken::SemiColon);
<<EOF>>                 return Ok(ForthToken::End);
%%

    pub fn remain(&self) -> usize {
        self.current.clone().count()
    }
