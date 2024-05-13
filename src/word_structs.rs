use std::collections::HashSet;
use crate::word_processing::get_letter_set;
use std::hash::Hash;


#[derive(Eq, PartialEq, Clone)]
pub struct Word {
    pub word: String,
    pub letters: HashSet<String>,
}

impl Word {
    pub fn new(word: String) -> Word {
        let letters = get_letter_set(&word);
        Word { word, letters }
    }
}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.word.hash(state); // hash only the word attribute
    }
}
