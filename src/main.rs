use rust_forth_tokenizer::ForthTokenizer;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file> [file2 ...]", args[0]);
        std::process::exit(1);
    }

    for file in &args[1..] {
        let file_string = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("Failed to open file '{}': {}", file, e));

        for token in ForthTokenizer::new(&file_string) {
            println!("Token is: {:?}", token);
        }

        let tokenizer = ForthTokenizer::new(&file_string);
        let collected: Vec<_> = tokenizer.into_iter().collect();
        println!("Tokenized vector is: {:?}", collected);
    }
}
