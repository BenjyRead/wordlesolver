use crate::word_structs::{GreenLetter, GreyLetter, Word, YellowCharacter};
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

fn filter_by_yellow_letters(
    words: &HashSet<Word>,
    yellow_letters: &HashSet<YellowCharacter>,
) -> HashSet<Word> {
    let mut filtered_words = words.clone();

    for word in words {
        for yellow_letter in yellow_letters {
            if word
                .word
                .chars()
                .filter(|letter| letter == &yellow_letter.letter)
                .count()
                < yellow_letter.count as usize
            {
                filtered_words.remove(word);
            } else {
                for (index, letter) in word.word.chars().enumerate() {
                    if letter == yellow_letter.letter
                        && yellow_letter.not_positions.contains(&(index as u8))
                    {
                        filtered_words.remove(word);
                    }
                }
            }
        }
    }

    return filtered_words;
}

pub fn filter_words(
    words: &HashSet<Word>,
    grey_letters: &HashSet<GreyLetter>,
    green_letters: &HashSet<GreenLetter>,
    yellow_letters: &HashSet<YellowCharacter>,
) -> HashSet<Word> {
    let mut filtered_words = filter_by_grey_letters(words, grey_letters);
    filtered_words = filter_by_green_letters(&filtered_words, green_letters);
    filtered_words = filter_by_yellow_letters(&filtered_words, yellow_letters);

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
        test_filter_by_grey_letters_3, ["aaaaa"], ["a0"], [],
        test_filter_by_grey_letters_4, ["aaaaa"], ["a4"], []
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

    macro_rules! test_filter_by_yellow_letters {
        ($($function_name: ident, [$($word:expr), *], [$(($letter:expr, $not_positions:expr, $count:expr)), *], [$($answer:expr), *]), *) => {
            $(
                #[test]
                fn $function_name() {
                    let words = HashSet::from(
                        [$(Word::new($word.to_string())), *]
                    );

                    let yellow_letters = HashSet::from(
                        [$(YellowCharacter{letter: $letter, not_positions: HashSet::from($not_positions), count: $count}), *]
                    );

                    let answers = HashSet::from(
                        [$(Word::new($answer.to_string())), *]
                    );

                    let filtered_words = filter_by_yellow_letters(&words, &yellow_letters);

                    assert_eq!(filtered_words, answers);
                }
             )*
        }
    }

    test_filter_by_yellow_letters! {
        test_filter_by_yellow_letters_empty, [], [], [],
        test_filter_by_yellow_letters_no_yellow, ["hello"], [], ["hello"],
        test_filter_by_yellow_letters_0, ["hello"], [('h', [], 1)], ["hello"],
        test_filter_by_yellow_letters_1, ["hello"], [('h', [0], 1)], [],
        test_filter_by_yellow_letters_2, ["hello"], [('h', [1,2,3,4], 1)], ["hello"],
        test_filter_by_yellow_letters_3, ["hello"], [('l', [0,1,4], 2)], ["hello"],
        test_filter_by_yellow_letters_4, ["hello","world"], [('l', [0,1], 1)], ["hello", "world"],
        test_filter_by_yellow_letters_5, ["hello","world"], [('l', [0,1,3], 1)], []
    }

    macro_rules! test_filter_words {
        ($(
            $function_name: ident,
            [$($word:expr), *],
            [$($grey_letter:expr), *],
            [$(($green_letter:expr, $position:expr)), *],
            [$(($yellow_letter:expr, $not_positions:expr, $yellow_count:expr)), *],
            [$($answer:expr), *]), *) =>
        {
            $(
                #[test]
                fn $function_name() {
                    let words = HashSet::from(
                        [$(Word::new($word.to_string())), *]
                    );

                    let grey_letters = HashSet::from(
                        [$(GreyLetter{letter: $grey_letter.to_string()}), *]
                    );

                    let green_letters = HashSet::from(
                        [$(GreenLetter{letter: $green_letter, position: $position as u8}), *]
                    );

                    let yellow_letters = HashSet::from(
                        [$(YellowCharacter{letter: $yellow_letter, not_positions: HashSet::from($not_positions), count: $yellow_count}), *]
                    );

                    let answers = HashSet::from(
                        [$(Word::new($answer.to_string())), *]
                    );

                    let filtered_words = filter_words(&words, &grey_letters, &green_letters, &yellow_letters);

                    assert_eq!(filtered_words, answers);
                }
            )*
        }
    }

    test_filter_words! {
        test_filter_words_empty, [], [], [], [], [],
        test_filter_words_no_filters, ["hello"], [], [], [], ["hello"],
        test_filter_words_no_grey, ["hello"], [], [('h', 0)], [('h', [4], 1)], ["hello"],
        //TODO: is there a conflict between yellow and green filters?
        test_filter_words_1, ["hello", "world", "soare"], ["l0"], [], [], ["soare"],
        test_filter_words_2, ["hello", "world", "soare"], ["l1"], [('l', 3)], [('l', [1,2,4], 1)], ["world"]
    }
}
