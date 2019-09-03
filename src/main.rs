use rust_forth_tokenizer::ForthTokenizer;
use rust_forth_tokenizer::ForthToken;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    for file in &args[1..] {
        let file_string = fs::read_to_string(file).expect("Failure opening file");

        for token in ForthTokenizer::new(&file_string) {
            println!("Token is: {:?}", token);
        }
    }
}
