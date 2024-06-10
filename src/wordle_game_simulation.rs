use crate::filtering::filter_words;
use crate::word_processing::get_letter_vec;
use crate::word_structs::{GreenLetter, GreyLetter, Word, YellowCharacter};
use std::collections::HashSet;

use crate::word_processing::{get_highest_point_word, get_letter_distribution};

fn get_greens(answer: &Word, guess: &Word, current_string: &mut String) {
    for (position, letter) in guess.word.chars().enumerate() {
        if answer.word.chars().nth(position) == Some(letter) {
            //theres got to be a better way than x..=x
            current_string.replace_range(position..=position, "G");
        }
    }
}

//NOTE: Call after get_greens
fn get_yellows(answer: &Word, guess: &Word, current_string: &mut String) {
    let mut potential_yellows = answer.word.clone().replace("G", "_");

    for (position, guess_letter) in guess.word.chars().enumerate() {
        if current_string.chars().nth(position) != Some('G') {
            if let Some(potential_yellow_position) = potential_yellows.find(guess_letter) {
                current_string.replace_range(position..=position, "Y");
                potential_yellows
                    .replace_range(potential_yellow_position..=potential_yellow_position, "_");
            }
        }
    }
}

fn get_colors(answer: &Word, guess: &Word) -> String {
    //5 character string of greys
    let mut colors = String::from("ggggg");

    //NOTE: not commutative
    get_greens(answer, guess, &mut colors);
    get_yellows(answer, guess, &mut colors);

    return colors;
}

//TODO: write tests
fn store_colors(
    guess: &Word,
    colors: &str,
    green_letters: &mut HashSet<GreenLetter>,
    yellow_characters: &mut HashSet<YellowCharacter>,
    grey_letters: &mut HashSet<GreyLetter>,
) {
    for (position, letter) in colors.chars().enumerate() {
        match letter {
            'G' => {
                green_letters.insert(GreenLetter {
                    letter,
                    position: position as u8,
                });
            }
            'Y' => {
                let found: Option<YellowCharacter> = yellow_characters
                    .iter()
                    .find(|x| x.letter == letter)
                    .cloned();

                if let Some(mut yellow_character) = found {
                    //if yellow_character.count is lower than the actual count of letters that
                    //are yellow in the word, change it to the actual count

                    if let Ok(yellow_letter_count) = colors
                        .chars()
                        .enumerate()
                        .filter(|&(index, element)| {
                            element == 'Y'
                                && &guess.word.chars().nth(index) == &Some(yellow_character.letter)
                        })
                        .count()
                        .try_into()
                    {
                        if yellow_character.count < yellow_letter_count {
                            yellow_character.count = yellow_letter_count;
                        }
                    }

                    yellow_character.not_positions.insert(position as u8);
                } else {
                    yellow_characters.insert(YellowCharacter {
                        letter,
                        not_positions: vec![position as u8].into_iter().collect::<HashSet<u8>>(),
                        count: 1,
                    });
                }
            }
            'g' => {
                grey_letters.insert(GreyLetter {
                    letter: get_letter_vec(&guess.word)[position].clone(),
                });
            }
            _ => {
                panic!("Invalid color");
            }
        }
    }
}

pub fn simulate_game(
    answer: &Word,
    valid_guess_words: &HashSet<Word>,
    valid_answer_words: &HashSet<Word>,
) -> i8 {
    println!("Answer = {}", answer.word);

    let mut green_letters: HashSet<GreenLetter> = HashSet::new();
    let mut yellow_characters: HashSet<YellowCharacter> = HashSet::new();
    let mut grey_letters: HashSet<GreyLetter> = HashSet::new();

    let mut valid_guess_words = valid_guess_words.clone();
    let mut valid_answer_words = valid_answer_words.clone();

    let mut turn_count: u8 = 0;
    while turn_count < 6 {
        turn_count += 1;
        let mut guess_words_distribution = get_letter_distribution(&valid_guess_words);
        let mut answer_words_distribution = get_letter_distribution(&valid_answer_words);

        let mut guess = get_highest_point_word(
            &valid_guess_words,
            &get_letter_distribution(&valid_guess_words),
        );

        println!("Guess = {}", guess.word);

        if &guess == answer {
            print!("Answer = {}, in {} tries", guess.word, turn_count);
            return turn_count.try_into().unwrap();
        }

        let colors = get_colors(&answer, &guess);

        store_colors(
            &guess,
            &colors,
            &mut green_letters,
            &mut yellow_characters,
            &mut grey_letters,
        );

        println!("green letters = {:?}", green_letters);
        println!("yellow characters = {:?}", yellow_characters);
        println!("grey letters = {:?}", grey_letters);

        valid_guess_words = filter_words(
            &valid_guess_words,
            &grey_letters,
            &green_letters,
            &yellow_characters,
        );

        valid_answer_words = filter_words(
            &valid_answer_words,
            &grey_letters,
            &green_letters,
            &yellow_characters,
        );
    }

    return -1;
}

mod tests {
    use super::*;

    macro_rules! test_get_greens {
        ($($function_name: ident, $guess: expr, $answer: expr, $current_string: expr, $solution: expr), *) => {
            $ (
                #[test]
                fn $function_name() {
                    let guess = Word::new($guess.to_string());
                    let answer = Word::new($answer.to_string());
                    let mut current_string = String::from($current_string);

                    get_greens(&answer, &guess, &mut current_string);

                    assert_eq!(current_string, $solution);
                }
            )*
        };
    }

    test_get_greens!(
        test_get_greens_1,
        "hello",
        "world",
        "     ",
        "   G ",
        test_get_greens_2,
        "hello",
        "hello",
        "     ",
        "GGGGG",
        test_get_greens_3,
        "hello",
        "hello",
        "ggggg",
        "GGGGG",
        test_get_greens_4,
        "hello",
        "world",
        "ggggg",
        "gggGg"
    );

    macro_rules! test_get_yellows {
        ($($function_name: ident, $guess: expr, $answer: expr, $current_string: expr, $solution: expr), *) => {
            $ (
                #[test]
                fn $function_name() {
                    let guess = Word::new($guess.to_string());
                    let answer = Word::new($answer.to_string());
                    let mut current_string = String::from($current_string);

                    get_yellows(&answer, &guess, &mut current_string);

                    assert_eq!(current_string, $solution);
                }
            )*
        };
    }

    test_get_yellows!(
        test_get_yellows_1,
        "hello",
        "world",
        "gggGg",
        "ggYGY",
        test_get_yellows_2,
        "hello",
        "hello",
        "GGGGG",
        "GGGGG",
        test_get_yellows_3,
        "aabbb",
        "bbaaa",
        "ggggg",
        "YYYYg",
        test_get_yellows_4,
        "hello",
        "zzzzz",
        "ggggg",
        "ggggg"
    );

    macro_rules! test_get_colors {
        ($($function_name: ident, $guess: expr, $answer: expr, $solution: expr), *) => {
            $ (
                #[test]
                fn $function_name() {
                    let guess = Word::new($guess.to_string());
                    let answer = Word::new($answer.to_string());

                    assert_eq!(get_colors(&answer, &guess), $solution);
                }
            )*
        };
    }

    test_get_colors! {
        test_get_colors_1, "hello", "world", "ggYGY",
        test_get_colors_2, "hello", "hello", "GGGGG",
        test_get_colors_3, "aabbb", "bbaaa", "YYYYg",
        test_get_colors_4, "hello", "zzzzz", "ggggg"
    }
}
