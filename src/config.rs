use std::cell::RefCell;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    num_words: usize,
    capitalization_enabled: bool,
    punctuations_enabled: bool,
    digits_enabled: bool,
    wordlist: RefCell<PathBuf>,
}

impl Config {
    pub fn get_num_words(&self) -> usize {
        self.num_words
    }

    pub fn get_capitalization(&self) -> bool {
        self.capitalization_enabled
    }

    pub fn get_punctuations(&self) -> bool {
        self.punctuations_enabled
    }

    pub fn get_digits(&self) -> bool {
        self.digits_enabled
    }

    pub fn get_wordlist(&self) -> String {
        self.wordlist.borrow().to_string_lossy().to_string()
    }

    pub fn set_num_words(&mut self, num_words: String) {
        if num_words.parse::<usize>().is_ok() {
            self.num_words = num_words.parse::<usize>().unwrap();
        } else if num_words.is_empty() {
            self.num_words = 0;
        }
    }

    pub fn toggle_capitalization(&mut self) {
        self.capitalization_enabled = !self.capitalization_enabled;
    }

    pub fn toggle_punctuations(&mut self) {
        self.punctuations_enabled = !self.punctuations_enabled;
    }

    pub fn toggle_digits(&mut self) {
        self.digits_enabled = !self.digits_enabled;
    }

    pub fn set_wordlist(&self, wordlist_path: Option<PathBuf>) {
        if let Some(wordlist) = wordlist_path {
            self.wordlist.replace(wordlist);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            num_words: 0,
            capitalization_enabled: false,
            punctuations_enabled: false,
            digits_enabled: false,
            wordlist: RefCell::new(PathBuf::new()),
        }
    }
}
