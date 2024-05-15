use crate::word_structs::{GreyLetter, Word};
use std::collections::HashSet;

pub fn filter_by_grey_letters(
    words: &HashSet<Word>,
    grey_letters: &HashSet<GreyLetter>,
) -> HashSet<Word> {
    let mut filtered_words = words.clone();

    for word in words {
        for grey_letter in grey_letters {
            if word.letters.contains(&grey_letter.letter) {
                filtered_words.remove(word);
            }
        }
    }
    return filtered_words;
}

mod tests {
    use super::*;

    macro_rules! test_filter_by_grey_letters {
        ($($function_name: ident, [$($word:expr), *], [$($grey_letter:expr), *], [$($answer:expr), *]), *) => {
            $(
                #[test]
                fn $function_name() {
                    let words = HashSet::from(
                        [$(Word::new($word.to_string())), *]
                    );

                    let grey_letters = HashSet::from(
                        [$(GreyLetter{letter: $grey_letter.to_string()}), *]
                    );

                    let answers = HashSet::from(
                        [$(Word::new($answer.to_string())), *]
                    );

                    let filtered_words = filter_by_grey_letters(&words, &grey_letters);

                    assert_eq!(filtered_words, answers);
                }
             )*
        }
    }

    test_filter_by_grey_letters! {
        test_filter_by_grey_letters_empty, [], [], [],
        test_filter_by_grey_letters_no_grey, ["hello"], [], ["hello"],
        test_filter_by_grey_letters_0, ["hello"], ["l0"], [],
        test_filter_by_grey_letters_1, ["hello"], ["l1"], [],
        test_filter_by_grey_letters_2, ["aaaaa"], ["a0"], []
    }
}
