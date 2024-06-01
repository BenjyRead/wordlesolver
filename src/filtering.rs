use crate::word_structs::{GreenLetter, GreyLetter, Word};
use std::collections::HashSet;

fn filter_by_grey_letters(
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

fn filter_by_green_letters(
    words: &HashSet<Word>,
    green_letters: &HashSet<GreenLetter>,
) -> HashSet<Word> {
    let mut filtered_words = words.clone();

    for word in words {
        for green_letter in green_letters {
            if word.word.chars().nth(green_letter.position as usize) != Some(green_letter.letter) {
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
        test_filter_by_grey_letters_2, ["hello"], ["l2"], ["hello"],
        test_filter_by_grey_letters_3, ["aaaaa"], ["a0"], []
    }

    macro_rules! test_filter_by_green_letters {
        ($($function_name: ident, [$($word:expr), *], [$(($letter:expr, $position:expr)), *], [$($answer:expr), *]), *) => {
            $(
                #[test]
                fn $function_name() {
                    let words = HashSet::from(
                        [$(Word::new($word.to_string())), *]
                    );

                    let green_letters = HashSet::from(
                        [$(GreenLetter{letter: $letter, position: $position as u8}), *]
                    );

                    let answers = HashSet::from(
                        [$(Word::new($answer.to_string())), *]
                    );

                    let filtered_words = filter_by_green_letters(&words, &green_letters);

                    assert_eq!(filtered_words, answers);
                }
             )*
        }
    }

    test_filter_by_green_letters! {
        test_filter_by_green_letters_empty, [], [], [],
        test_filter_by_green_letters_no_green, ["hello"], [], ["hello"],
        test_filter_by_green_letters_0, ["hello"], [('h', 0)], ["hello"],
        test_filter_by_green_letters_1, ["hello"], [('h', 1)], [],
        test_filter_by_green_letters_2, ["hello"], [('e',1)], ["hello"],
        test_filter_by_green_letters_3, ["hello", "world"], [('o', 4)], ["hello"],
        test_filter_by_green_letters_4, ["hello", "world"], [('o', 1)], ["world"],
        test_filter_by_green_letters_5, ["hello", "world"], [('l', 3)], ["hello", "world"]
    }
}
