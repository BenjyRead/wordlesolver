use crate::word_processing::get_letter_set;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;

//NOTE: Don't really know why I need to include some of these
#[derive(Eq, PartialEq, Clone)]
pub struct Word {
    pub word: String,
    pub letters: HashSet<String>,
}

impl Word {
    pub fn new(word: String) -> Word {
        let letters = get_letter_set(&word);
        return Word { word, letters };
    }
}

impl Hash for Word {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.word.hash(state); // hash only the word attribute
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Word ( {} )", self.word)
    }
}

#[derive(Eq, Clone, Debug)]
pub struct GreyLetter {
    // TODO: ensure exactly 2 characers
    pub letter: String,
}

impl PartialEq for GreyLetter {
    fn eq(&self, other: &GreyLetter) -> bool {
        return self.letter == other.letter;
    }
}

impl Hash for GreyLetter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        return self.letter.hash(state); // hash only the letter attribute
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct GreenLetter {
    pub letter: char,
    //NOTE: 0-indexed
    pub position: u8,
}

#[derive(Eq, Clone, Debug)]
pub struct YellowCharacter {
    pub letter: char,
    pub not_positions: HashSet<u8>,
    //NOTE: 0-indexed, can be negative because the not_positions are still valuable information
    //even when we dont know any yellows are in the position
    pub count: i8,
}

impl Hash for YellowCharacter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.letter.hash(state); // hash only the letter attribut
    }
}

impl PartialEq for YellowCharacter {
    fn eq(&self, other: &YellowCharacter) -> bool {
        return self.letter == other.letter;
    }
}

mod tests {
    use super::*;

    macro_rules! test_new_word {
        ($($function_name: ident, $word: expr, [$($letter: expr), *]), *) => {
            $(
                #[test]
                fn $function_name() {
                    let word = Word::new($word.to_string());
                    assert_eq!(word.word, $word.to_string());
                    assert_eq!(word.letters, HashSet::from([$(String::from($letter)), *]));
                }
            )*
        };
    }

    test_new_word! {
        test_new_word_hello, "hello", ["h0", "e0", "l0", "l1", "o0"],
        test_new_word_world, "world", ["w0", "o0", "r0", "l0", "d0"],
        test_new_word_lllll, "lllll", ["l0", "l1", "l2", "l3", "l4"]
    }
}
