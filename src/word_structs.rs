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

mod tests {
    use super::*;

    #[test]
    fn test_word_new() {
        let word = Word::new("hello".to_string());
        assert_eq!(word.word, "hello");
        assert_eq!(word.letters,
                   HashSet::from_iter(
                       ["h0", "e0", "l0", "l1", "o0"]
                           .iter()
                           .map(|s| String::from(*s))
                    ));
    }
}
