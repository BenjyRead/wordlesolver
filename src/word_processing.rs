use crate::word_structs::Word;
use std::collections::{HashMap, HashSet};

fn string_counter(word: &str) -> HashMap<char, u32> {
    return word.chars().fold(HashMap::new(), |mut counter, c| {
        *counter.entry(c).or_insert(0) += 1;
        return counter;
    });
}

pub fn char_count(word: &str, letter: &char) -> u32 {
    return word.chars().filter(|&c| c == *letter).count() as u32;
}

//Mostly repeat code frmo get_letter_set
pub fn get_letter_vec(word: &str) -> Vec<String> {
    if word.len() != 5 {
        panic!("Word must be 5 characters long");
    }

    let mut letters = Vec::new();

    for (a, letter) in word.chars().enumerate() {
        let num_code = char_count(&word[0..=a], &letter) - 1;

        letters.push(letter.to_string() + &num_code.to_string());
    }

    return letters;
}

pub fn get_letter_set(word: &str) -> HashSet<String> {
    if word.len() != 5 {
        panic!("Word must be 5 characters long");
    }

    let counter = string_counter(word);

    let mut letters = HashSet::new();

    for (letter, count) in counter {
        for a in 0..count {
            letters.insert(format!("{}{}", letter, a));
        }
    }

    return letters;
}

pub fn get_letter_distribution(words: &HashSet<Word>) -> HashMap<String, u32> {
    let mut letter_distribution = HashMap::new();

    for word in words {
        let letters = &word.letters;
        for letter in letters {
            *letter_distribution.entry(letter.clone()).or_insert(0) += 1;
        }
    }

    return letter_distribution;
}

fn get_word_points(word: &Word, distribution: &HashMap<String, u32>) -> u32 {
    let mut points = 0;

    for letter in &word.letters {
        points += distribution.get(letter).unwrap();
    }

    return points;
}

pub fn get_highest_point_word(words: &HashSet<Word>, distribution: &HashMap<String, u32>) -> Word {
    let mut highest_point_word: Option<&Word> = None;
    let mut highest_points = 0;

    for word in words {
        let points = get_word_points(&word, &distribution);

        if points > highest_points {
            highest_points = points;
            highest_point_word = Some(&word);
        }
    }

    if highest_point_word.is_none() {
        panic!("No words in the set");
    } else {
        return highest_point_word.unwrap().clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_counter() {
        let test_string_counter_test_cases = [
            (
                "hello",
                [('h', 1), ('e', 1), ('l', 2), ('o', 1)] // generate a hashmap
                    .into_iter()
                    .collect(),
            ),
            (
                "world",
                [('w', 1), ('o', 1), ('r', 1), ('l', 1), ('d', 1)]
                    .into_iter()
                    .collect(),
            ),
        ];

        for (word, hashmap) in test_string_counter_test_cases {
            assert_eq!(string_counter(word), hashmap);
        }
    }

    macro_rules! test_get_letter_set {
        ($($function_name: ident, $word: expr, [$($letter:expr), *]), *) => {
            $(
                #[test]
                fn $function_name() {
                    let letters = HashSet::from([$(String::from($letter)), *]);


                    assert_eq!(get_letter_set($word), letters);
                }
            )*
        }
    }

    macro_rules! test_get_letter_set_error {
        ($($function_name: ident, $word: expr, $panic_message: expr), *) => {
            $(
                #[test]
                #[should_panic(expected = $panic_message)]
                fn $function_name() {
                    get_letter_set($word);
                }
            )*
        }
    }

    test_get_letter_set! {
        test_get_letter_set_hello, "hello", ["h0", "e0", "l0", "l1", "o0"],
        test_get_letter_set_world, "world", ["w0", "o0", "r0", "l0", "d0"],
        test_get_letter_set_lllll, "lllll", ["l0", "l1", "l2", "l3", "l4"]
    }

    test_get_letter_set_error! {
        test_get_letter_set_error_empty, "", "Word must be 5 characters long",
        test_get_letter_set_error_4_characters, "hell", "Word must be 5 characters long",
        test_get_letter_set_error_6_characters, "helloo", "Word must be 5 characters long"
    }

    //TODO: macro and more tests

    #[test]
    fn test_get_letter_distribution() {
        let test_get_letter_distribution_test_cases = [(
            HashSet::from([
                Word::new(String::from("hello")),
                Word::new(String::from("world")),
            ]),
            HashMap::<String, u32>::from_iter(
                [
                    ("h0", 1),
                    ("e0", 1),
                    ("l0", 2),
                    ("l1", 1),
                    ("o0", 2),
                    ("w0", 1),
                    ("r0", 1),
                    ("d0", 1),
                ]
                .into_iter()
                .map(|(k, v)| (k.to_string(), v)),
            ),
        )];

        for (words, distribution) in test_get_letter_distribution_test_cases {
            assert_eq!(get_letter_distribution(&words), distribution);
        }
    }

    macro_rules! test_get_word_points{
        ($($function_name:ident, $word:expr, [ $(($letter2char:expr,$letter_points:expr)), *], $expected_points:expr), + ) =>{
            $(

                #[test]
                fn $function_name(){
                    let word = Word::new(String::from($word));

                    let distribution: HashMap::<String, u32> = HashMap::from_iter(
                        [ $(($letter2char.to_string(), $letter_points)), * ]
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v)),
                    );

                    assert_eq!(get_word_points(&word, &distribution), $expected_points);
                }

            )*
        };
    }

    test_get_word_points!(
        test_get_word_points_0,
        "hello",
        [
            ("h0", 1),
            ("e0", 1),
            ("l0", 2),
            ("l1", 1),
            ("o0", 2),
            ("w0", 1),
            ("r0", 1),
            ("d0", 1)
        ],
        7,
        test_get_word_points_1,
        "world",
        [
            ("h0", 1),
            ("e0", 1),
            ("l0", 2),
            ("l1", 1),
            ("o0", 2),
            ("w0", 1),
            ("r0", 1),
            ("d0", 1)
        ],
        7
    );

    macro_rules! test_get_highest_point_word{
        ($($function_name:ident, [$($word_string:expr), *] , [ $(($letter2char:expr,$letter_points:expr)), *], $expected_word:expr), + ) =>{
            $(

                #[test]
                fn $function_name(){
                    let words = HashSet::from_iter(
                        [ $(
                            Word::new(String::from($word_string))
                            )
                        , *
                        ]
                        .into_iter()
                    );

                    let distribution: HashMap::<String, u32> = HashMap::from_iter(
                        [ $(($letter2char.to_string(), $letter_points)), * ]
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v)),
                    );

                    assert_eq!(get_highest_point_word(&words, &distribution).word, $expected_word);
                }

            )*
        };
    }

    test_get_highest_point_word!(
        test_get_highest_point_word_0,
        ["hello"],
        [
            ("h0", 1),
            ("e0", 1),
            ("l0", 2),
            ("l1", 1),
            ("o0", 2),
            ("w0", 1),
            ("r0", 1),
            ("d0", 1)
        ],
        "hello",
        test_get_highest_point_word_1,
        ["hello", "world", "ellow"],
        [
            ("h0", 1),
            ("e0", 2),
            ("l0", 3),
            ("l1", 2),
            ("o0", 3),
            ("w0", 2),
            ("r0", 1),
            ("d0", 1),
            ("y0", 1)
        ],
        "ellow" //TODO: make more better tests
    );

    macro_rules! test_char_count {
        ($($function_name:ident, $word: expr, $char: expr, $count: expr), *) => {
            $(
                #[test]
                fn $function_name() {
                    assert_eq!(char_count(&$word, &$char), $count);
                }
            )*
        };
    }

    test_char_count!(
        test_char_count_0,
        "hello",
        'l',
        2,
        test_char_count_1,
        "aaaaa",
        'a',
        5,
        test_char_count_2,
        "world",
        'z',
        0
    );

    macro_rules! test_get_letter_vec {
        ($($function_name: ident, $word: expr, $answer_vec: expr), *) => {
            $(
                #[test]
                fn $function_name() {
                    assert_eq!(get_letter_vec($word), $answer_vec);
                }
            )*
        }
    }

    test_get_letter_vec!(
        test_get_letter_vec_0,
        "hello",
        vec!["h0", "e0", "l0", "l1", "o0"],
        test_get_letter_vec_1,
        "world",
        vec!["w0", "o0", "r0", "l0", "d0"],
        test_get_letter_vec_2,
        "lllll",
        vec!["l0", "l1", "l2", "l3", "l4"]
    );
}
