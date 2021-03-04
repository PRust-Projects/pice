use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

use inflector::Inflector;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::config::Config;

const PUNCTUATIONS: &[&str] = &[
    "~", "!", "#", "$", "%", "^", "&", "*", "(", ")", "-", "=", "+", "[", "]", "\\", "{", "}", ":",
    ";", "\"", "'", "<", ">", "?", "/",
];

pub fn generate(config: &Config) -> Result<String> {
    let wordlist = load_wordlist(&config.get_wordlist_path())?;
    let mut rng = rand::thread_rng();

    let mut components = Vec::new();
    let mut should_capitalize = false;
    for _ in 0..config.get_num_words() {
        match rng.gen_range(0..6) {
            0 => {
                if config.get_digits() {
                    let digit = rng.gen_range(0..10);
                    components.push(digit.to_string());
                }
            }
            1 => {
                if config.get_punctuations() {
                    let punct = PUNCTUATIONS.choose(&mut rng).unwrap();
                    components.push(punct.to_string());
                }
            }
            2 => {
                should_capitalize = config.get_capitalization();
            }
            _ => {}
        }

        let mut word = wordlist.choose(&mut rng).unwrap().to_string();
        if should_capitalize {
            word = word.to_title_case();
            should_capitalize = false;
        }
        components.push(word.clone());
    }
    println!("{:?}", components);
    Ok(components.join(" "))
}

pub fn load_wordlist<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut wordlist = vec![];
    for line in buffered.lines() {
        let line = line?;
        match line.find("\t") {
            Some(i) => wordlist.push(line[i + 1..].to_string()),
            None => wordlist.push(line),
        }
    }

    Ok(wordlist)
}
