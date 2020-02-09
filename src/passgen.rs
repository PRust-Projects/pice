extern crate inflections;
extern crate rand;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use inflections::Inflect;
use rand::Rng;

const INTERAL_ERR_STRING: &'static str = "Internal error.";
const PUNCTUATIONS: &[&str] = &["~", "!", "#", "$", "%", "^", "&", "*", "(", ")", "-", "=", "+", "[", "]", "\\", "{", "}", ":", ";", "\"", "'", "<", ">", "?", "/"];

#[derive(Debug)]
pub struct PasswordConfig {
    num_words: u64,
    capitalization_enabled: bool,
    punctuation_enabled: bool,
    number_enabled: bool,
    wordlist: String,
}

pub fn config_from_sciter_value(data: sciter::Value) -> Result<PasswordConfig, String> {
    let num_words = data.get_item("num-words").to_int().ok_or(INTERAL_ERR_STRING)?;
    let caps_enabled = data.get_item("capitalization-enabled").to_bool().ok_or(INTERAL_ERR_STRING)?;
    let punct_enabled = data.get_item("punctuation-enabled").to_bool().ok_or(INTERAL_ERR_STRING)?;
    let num_enabled = data.get_item("number-enabled").to_bool().ok_or(INTERAL_ERR_STRING)?;
    let wordlist = data.get_item("custom-wordlist").as_string().ok_or(INTERAL_ERR_STRING)?;
    
    Ok(PasswordConfig{
        num_words: num_words as u64,
        capitalization_enabled: caps_enabled,
        punctuation_enabled: punct_enabled,
        number_enabled: num_enabled,
        wordlist: wordlist,
    })
}

pub fn generate(config: PasswordConfig) -> Result<String, Error> {
    let wordlist = load_wordlist(&config.wordlist)?;
    let mut rng = rand::thread_rng();

    let mut components = Vec::new();
    let mut should_capitalize = false;
    for _ in 0..config.num_words {
        match rng.gen_range(0, 6) {
            0 => {
                if config.number_enabled {
                    let digit = rng.gen_range(0, 10);
                    components.push(digit.to_string());
                }
            },
            1 => {
                if config.punctuation_enabled {
                    let punct = PUNCTUATIONS[rng.gen_range(0, PUNCTUATIONS.len())];
                    components.push(punct.to_string());
                }
            },
            2 => {
                should_capitalize = config.capitalization_enabled;
            },
            _ => {},
        }

        let mut word = wordlist[rng.gen_range(0, wordlist.len())].clone();
        if should_capitalize {
            word = word.to_title_case();
            should_capitalize = false;
        }
        components.push(word.clone());
    }
    println!("{:?}", components);
    Ok(components.join(" "))
}

fn load_wordlist(path: &str) -> Result<Vec<String>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut wordlist = Vec::new();
    for line in buffered.lines() {
        let line = line?;
        match line.find("\t") {
            Some(i) => wordlist.push(line[i+1..].to_string()),
            None => return Err(Error::new(ErrorKind::Other, "wordlist is in invalid format")),
        };
    }
    Ok(wordlist)
}
