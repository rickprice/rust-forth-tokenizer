# rust-forth-tokenizer
A Forth tokenizer written in Rust

Usage:

main() {
use rust_forth_tokenizer::ForthToken;
use rust_forth_tokenizer::ForthTokenizer;

     let tokenizer = ForthTokenizer::new("word : wordname 1 2 3 ; definition");
     // The code also supports the regular for loop iterator syntax
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