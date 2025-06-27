use std::collections::{HashMap, HashSet};

use regex::Regex;

use anyhow::{Error, Result};

struct Vocab {
    data: HashMap<String, usize>,
}

impl Vocab {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn extend_from_text(&mut self, input: &str) {
        let re = Regex::new(r#"[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*|[,.?_!"()':;]|--|\s"#)
            .expect("Provided regex was tested and should be correct");

        let unique_tokens: HashSet<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
        let mut unique_tokens = Vec::from_iter(unique_tokens);

        unique_tokens.sort();

        let tokens_to_token_ids: Vec<(String, usize)> = unique_tokens
            .iter()
            .enumerate()
            .map(|(idx, elem)| ((*elem).to_string(), idx))
            .collect();

        self.data.extend(tokens_to_token_ids);
    }
}

struct SimpleTokenizerV1 {
    token_to_token_ids: HashMap<String, usize>,
    token_ids_to_tokens: HashMap<usize, String>,
}

impl SimpleTokenizerV1 {
    fn new(vocab: Vocab) -> Self {
        Self {
            token_to_token_ids: vocab.data.clone(),
            token_ids_to_tokens: vocab
                .data
                .into_iter()
                .map(|(text, token)| (token, text))
                .collect(),
        }
    }

    fn encode(&self, input: &str) -> Result<Vec<usize>> {
        let re = Regex::new(r#"[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*|[,.?_!"()':;]|--|\s"#)
            .expect("Provided regex was tested and should be correct");

        let tokens: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

        tokens
            .iter()
            .map(|&text| {
                self.token_to_token_ids
                    .get(text)
                    .copied()
                    .ok_or(Error::msg("Token not found in vocabulary"))
            })
            .collect()
    }

    fn decode(&self, ids: Vec<usize>) -> Result<String> {
        let tokens: Vec<String> = ids
            .iter()
            .map(|id| {
                self.token_ids_to_tokens
                    .get(id)
                    .ok_or(Error::msg(format!(
                        "Failed to find token for provided token id: {}",
                        id
                    )))
                    .cloned()
            })
            .collect::<Result<Vec<String>, Error>>()?;
        Ok(tokens.join(""))
    }
}

fn main() -> Result<()> {
    let input = include_str!("../the-verdict.txt");
    println!("Num of chars: {}", input.len());

    let test_text = "Hello, world. Is this-- a test?";

    let mut vocab = Vocab::new();
    vocab.extend_from_text(input);

    let tokenizer = SimpleTokenizerV1::new(vocab);

    let tokens = tokenizer.encode(input)?;

    let decoded = tokenizer.decode(tokens)?;

    println!("{}", decoded);
    assert_eq!(decoded, input);

    Ok(())
}
