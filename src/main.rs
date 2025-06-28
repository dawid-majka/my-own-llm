use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../the-verdict.txt");
    println!("Num of chars: {}", input.len());

    let test_text = "Hello, world. Is this-- a test?";

    let tokenizer = tiktoken_rs::get_bpe_from_model("gpt2").unwrap();

    let text1 = "Hello, do you like tea?";
    let text2 = "In the sunlit terraces of the palace.";
    let text = format!("{} <|endoftext|> {}", text1, text2);

    let allowed_specials = HashSet::from(["<|endoftext|>"]);

    let (tokens, _) = tokenizer.encode(&text, &allowed_specials);

    println!("{:?}", tokens);

    let decoded = tokenizer.decode(tokens)?;

    println!("{}", decoded);
    assert_eq!(decoded, text);

    Ok(())
}
