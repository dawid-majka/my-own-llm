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
        let re =
            Regex::new(r#"<\|[a-zA-Z0-9_]+\|>|[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*|[,.?_!"()':;]|--|\s"#)
                .expect("Provided regex was tested and should be correct");

        let unique_tokens: HashSet<&str> = re.find_iter(input).map(|m| m.as_str()).collect();
        let mut unique_tokens = Vec::from_iter(unique_tokens);

        unique_tokens.sort();

        unique_tokens.push("<|unk|>");
        unique_tokens.push("<|endoftext|>");

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
        let re =
            Regex::new(r#"<\|[a-zA-Z0-9_]+\|>|[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*|[,.?_!"()':;]|--|\s"#)
                .expect("Provided regex was tested and should be correct");

        let tokens: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

        tokens
            .iter()
            .map(|&text| {
                self.token_to_token_ids
                    .get(text)
                    .or(self.token_to_token_ids.get("<|unk|>"))
                    .ok_or(Error::msg(
                        "Vocabulary does not contain token id for unknown token",
                    ))
                    .cloned()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_tokenizer_from_text(text: &str) -> SimpleTokenizerV1 {
        let mut vocab = Vocab::new();
        vocab.extend_from_text(text);
        SimpleTokenizerV1::new(vocab)
    }

    #[test]
    fn test_encode_decode_simple_sentence() {
        let input = "Hello, world. Is this-- a test?";
        let tokenizer = setup_tokenizer_from_text(input);

        let tokens = tokenizer.encode(input).expect("Encoding failed");
        let decoded = tokenizer.decode(tokens).expect("Decoding failed");

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_encode_decode_with_special_tokens() {
        let input = "Hello, do you like tea? <|endoftext|> In the sunlit terraces of the palace.";
        let tokenizer = setup_tokenizer_from_text(input);

        let tokens = tokenizer.encode(input).expect("Encoding failed");
        let decoded = tokenizer.decode(tokens).expect("Decoding failed");

        assert_eq!(decoded, input);
    }

    #[test]
    fn test_unknown_token_handling() {
        let input = "Hello, world. Is this-- a test?";
        let tokenizer = setup_tokenizer_from_text(input);

        let unknown_text =
            "Hello, do you like tea? <|endoftext|> In the sunlit terraces of the palace.";
        let tokens = tokenizer.encode(unknown_text).expect("Encoding failed");

        let unk_id = tokenizer.token_to_token_ids.get("<|unk|>").unwrap();
        assert_eq!(tokens.iter().filter(|id| *id == unk_id).count(), 11);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let tokenizer = setup_tokenizer_from_text(input);

        let tokens = tokenizer.encode("").expect("Encoding failed");
        assert!(tokens.is_empty());

        let decoded = tokenizer.decode(tokens).expect("Decoding failed");
        assert_eq!(decoded, "");
    }
}
