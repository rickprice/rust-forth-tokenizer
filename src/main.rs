mod test {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    for file in &args[1..] {
        let file_string = fs::read_to_string(file).expect("Failure opening file");

        let mut lex = test::Lexer::new(&file_string, test::SpaceCounter::new());
        loop {
            let res = lex.yylex();
            println!("{:?}", res);
            if lex.is_eof() {
                break;
            }
/*            
            if res.is_err() {
                break;
            }
*/
            println!("remain '{}' characters", lex.remain());
        }
        println!("space count: {}", lex.get_space_counter().count());
    }
}
