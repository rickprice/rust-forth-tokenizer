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


#[derive(Debug)]
pub enum Error {
    EOF,
    Unmatch,
}

pub struct Lexer<'a> {
    cmap: Vec<usize>,
    start: std::str::Chars<'a>,
    current: std::str::Chars<'a>,
    max_len: usize,


    zz_state: usize,
    zz_lexical_state: usize,
    zz_marked_pos: usize,
    zz_current_pos: usize,
    zz_start_read: usize,

    zz_at_eof: bool,

    space_counter: SpaceCounter,
}

impl<'a> Lexer<'a> {
    pub const ZZ_ROW: [usize; 14] = [0, 9, 18, 27, 36, 45, 54, 18, 9, 27, 18, 63, 36, 63];
    pub const ZZ_TRANS: [i32; 72] = [1, 2, 3, 4, 5, 2, 6, 5, 7, 8, -1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 9, 10, 9, 9, 9, 9, 9, 9, 9, 11, -1, 11, 4, 12, 11, 4, 4, 11, -1, -1, -1, 5, 5, -1, 5, 5, -1, 9, 10, 9, 6, 6, 9, 6, 6, 9, 11, -1, 11, 11, 13, 11, 11, 11, 11];
    pub const ZZ_ATTR: [i32; 14] = [0, 0, 9, 1, 1, 1, 1, 9, 1, 0, 9, 0, 1, 1];
    pub const ZZ_ACTION: [i32; 14] = [0, 0, 1, 2, 3, 4, 5, 6, 7, 0, 8, 0, 9, 10];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str, space_counter: SpaceCounter) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let chars = input.chars();
        let mut cmap: Vec<usize> = Vec::with_capacity(0x110000);
        cmap.resize(0x110000, 0);
        cmap[9] = 5;
        cmap[10] = 1;
        cmap[11] = 1;
        cmap[12] = 1;
        cmap[13] = 1;
        cmap[32] = 5;
        cmap[35] = 6;
        cmap[36] = 6;
        cmap[45] = 7;
        cmap[46] = 6;
        cmap[48] = 7;
        cmap[49] = 7;
        cmap[50] = 7;
        cmap[51] = 7;
        cmap[52] = 7;
        cmap[53] = 7;
        cmap[54] = 7;
        cmap[55] = 7;
        cmap[56] = 7;
        cmap[57] = 7;
        cmap[58] = 8;
        cmap[59] = 2;
        cmap[63] = 6;
        cmap[64] = 6;
        cmap[65] = 6;
        cmap[66] = 6;
        cmap[67] = 6;
        cmap[68] = 6;
        cmap[69] = 6;
        cmap[70] = 6;
        cmap[71] = 6;
        cmap[72] = 6;
        cmap[73] = 6;
        cmap[74] = 6;
        cmap[75] = 6;
        cmap[76] = 6;
        cmap[77] = 6;
        cmap[78] = 6;
        cmap[79] = 6;
        cmap[80] = 6;
        cmap[81] = 6;
        cmap[82] = 6;
        cmap[83] = 6;
        cmap[84] = 6;
        cmap[85] = 6;
        cmap[86] = 6;
        cmap[87] = 6;
        cmap[88] = 6;
        cmap[89] = 6;
        cmap[90] = 6;
        cmap[92] = 6;
        cmap[95] = 6;
        cmap[97] = 6;
        cmap[98] = 6;
        cmap[99] = 6;
        cmap[100] = 6;
        cmap[101] = 6;
        cmap[102] = 6;
        cmap[103] = 6;
        cmap[104] = 6;
        cmap[105] = 6;
        cmap[106] = 6;
        cmap[107] = 6;
        cmap[108] = 6;
        cmap[109] = 6;
        cmap[110] = 6;
        cmap[111] = 6;
        cmap[112] = 6;
        cmap[113] = 6;
        cmap[114] = 6;
        cmap[115] = 6;
        cmap[116] = 6;
        cmap[117] = 6;
        cmap[118] = 6;
        cmap[119] = 6;
        cmap[120] = 6;
        cmap[121] = 6;
        cmap[122] = 6;
        cmap[123] = 3;
        cmap[125] = 4;
        cmap[133] = 1;
        cmap[8232] = 1;
        cmap[8233] = 1;


        Lexer {
            cmap,
            start: chars.clone(),
            current: chars.clone(),

            max_len,
            zz_state: 0,
            zz_lexical_state: Lexer::YYINITIAL,
            zz_marked_pos: 0,
            zz_current_pos: 0,
            zz_start_read: 0,

            zz_at_eof: false,

            space_counter,
        }
    }

        pub fn get_space_counter(&mut self) -> &mut SpaceCounter { &mut self.space_counter }

    pub fn is_eof(&self) -> bool {
        self.zz_at_eof
    }

    pub fn yybegin(&mut self, new_state: usize) {
        self.zz_lexical_state = new_state;
    }

    pub fn yystate(&self) -> usize {
        self.zz_lexical_state
    }

    pub fn yylength(&self) -> usize {
        self.zz_marked_pos - self.zz_start_read
    }

    pub fn yycharat(&self, pos: usize) -> Option<char> {
        self.start.clone().nth(pos)
    }

    pub fn yytext(&self) -> String {
        let len = self.zz_marked_pos - self.zz_start_read;
        let mut text = String::with_capacity(len);
        let mut chars = self.start.clone();

        for _ in 0..len {
            text.push(match chars.next() { Some(c) => c, _ => break,});
        }
        text
    }

    pub fn yypushback(&mut self, num: usize) {
        if num <= self.yylength() {
            self.zz_marked_pos -= num;
        }
    }

    pub fn yylex(&mut self) -> Result<ForthToken, Error> {
        let mut zz_input: i32;

        // cached
        loop {
            let mut zz_marked_pos_l = self.zz_marked_pos;
            let mut zz_action = -1;
            let mut zz_current_pos_l = self.zz_marked_pos;
            let mut current = self.current.clone();
            

            self.zz_start_read = self.zz_marked_pos;
            self.zz_current_pos = self.zz_marked_pos;
            self.start = self.current.clone();

            self.zz_state = Lexer::ZZ_LEXSTATE[self.zz_lexical_state] as usize;

            // set up zz_action for empty match case:
            let zz_attributes = Lexer::ZZ_ATTR[self.zz_state];
            if (zz_attributes & 1) == 1 {
                zz_action = self.zz_state as i32;
            }

            'zz_for_action: loop {
                if zz_current_pos_l < self.max_len {
                    zz_input = current.next().unwrap() as i32;
                    zz_current_pos_l += 1;
                } else if self.zz_at_eof {
                    zz_input = Lexer::YYEOF;
                    break 'zz_for_action;
                } else {
                    self.zz_current_pos = zz_current_pos_l;

                    if self.max_len <= zz_current_pos_l {
                        zz_input = Lexer::YYEOF;
                        break 'zz_for_action;
                    } else {
                        zz_input = current.next().unwrap() as i32;
                        zz_current_pos_l += 1;
                    }
                }

                let idx = Lexer::ZZ_ROW[self.zz_state] + self.cmap[zz_input as usize];
                let zz_next = Lexer::ZZ_TRANS[idx];
                if zz_next == -1 {
                    break 'zz_for_action;
                }
                self.zz_state = zz_next as usize;

                let zz_attributes = Lexer::ZZ_ATTR[self.zz_state];
                if (zz_attributes & 1) == 1 {
                    zz_action = self.zz_state as i32;
                    zz_marked_pos_l = zz_current_pos_l;
                    self.current = current.clone();
                    if (zz_attributes & 8) == 8 {
                        break 'zz_for_action;
                    }
                }
            }   // loop 'zz_for_action

            // store back cached position
            self.zz_marked_pos = zz_marked_pos_l;

            if zz_input == Lexer::YYEOF && self.zz_start_read == self.zz_current_pos {
                self.zz_at_eof = true;
                 match self.zz_lexical_state {
                     _ => { return Ok(ForthToken::End); }
                 }

                return Err(Error::EOF);
            } else {
                let action = if zz_action < 0 {
                    zz_action
                } else {
                    Lexer::ZZ_ACTION[zz_action as usize]
                };
                match action {
                    1 => { { self.space_counter.increment_space(); } }
                    11 => { /* nothing */ }
                    2 => { return Ok(ForthToken::SemiColon); }
                    12 => { /* nothing */ }
                    3 => { return Ok(ForthToken::ForthWord(self.yytext())); }
                    13 => { /* nothing */ }
                    4 => { return Ok(ForthToken::ForthWord(self.yytext())); }
                    14 => { /* nothing */ }
                    5 => { return Ok(ForthToken::ForthWord(self.yytext())); }
                    15 => { /* nothing */ }
                    6 => { return Ok(ForthToken::Colon); }
                    16 => { /* nothing */ }
                    7 => { return Ok(ForthToken::DeleteMeLocalDef(self.yytext().clone())); }
                    17 => { /* nothing */ }
                    8 => { return Ok(ForthToken::Comment(self.yytext().clone())); }
                    18 => { /* nothing */ }
                    9 => { return Ok(ForthToken::Comment(self.yytext().clone())); }
                    19 => { /* nothing */ }
                    10 => { return Ok(ForthToken::Comment(self.yytext().clone())); }
                    20 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

    pub fn remain(&self) -> usize {
        self.current.clone().count()
    }

}
