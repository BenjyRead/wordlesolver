use std::collections::HashSet;
use word_processing::get_letter_set;

struct Word {
    word: String,
    letters: HashSet<String>,
}

impl Word {
    fn new(word: String) -> Word {
        let letters = get_letter_set(&word);
        Word { word, letters }
    }
}
