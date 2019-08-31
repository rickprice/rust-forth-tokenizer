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

%%
%class Lexer
%result_type i32
%field SpaceCounter space_counter

\\.*$                  println!("Comment: '{}'", self.yytext()); return Ok(1i32);
;.*$                   println!("Comment2: '{}'", self.yytext()); return Ok(2i32);
{.*}                   println!("Comment3: '{}'", self.yytext()); return Ok(3i32);
\(.*\)                 println!("Local Variable Def: '{}'", self.yytext()); return Ok(4i32);
\n                     { self.space_counter.increment_space(); }
\t                     { self.space_counter.increment_space(); }
\r                     { self.space_counter.increment_space(); }
" "                    { self.space_counter.increment_space(); }
[a-zA-Z][a-zA-Z_0-9{}+\-\.#@$]*     println!("'{}'", self.yytext()); return Ok(10i32);
[0-9]+                 println!("'{}'", self.yytext()); return Ok(11i32);
<<EOF>>                println!("normal EOF");
%%

    pub fn remain(&self) -> usize {
        self.current.clone().count()
    }
