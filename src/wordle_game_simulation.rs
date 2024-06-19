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
    for (position, (color_letter, guess_letter)) in
        colors.chars().zip(guess.word.chars()).enumerate()
    {
        match color_letter {
            'G' => {
                green_letters.insert(GreenLetter {
                    letter: guess_letter,
                    position: position as u8,
                });

                let already_identified_as_yellow = yellow_characters
                    .iter()
                    .find(|x| x.letter == guess_letter)
                    .cloned();

                if let Some(mut yellow_letter) = already_identified_as_yellow {
                    //decrement the count of the yellow character, as one of them has been found
                    println!("{:?}", &yellow_letter);

                    yellow_letter.count -= 1;
                    //we will keep count = 0 yellow characters, as if a yellow character is found
                    //the not positions will still be useful
                    yellow_characters.replace(yellow_letter);
                }
            }

            'Y' | 'g' => {}

            _ => {
                panic!("Invalid color");
            }
        }
    }

    for (position, (color_letter, guess_letter)) in
        colors.chars().zip(guess.word.chars()).enumerate()
    {
        match color_letter {
            'G' => {
                // green_letters.insert(GreenLetter {
                //     letter: guess_letter,
                //     position: position as u8,
                // });
                //
                // let already_identified_as_yellow = yellow_characters
                //     .iter()
                //     .find(|x| x.letter == guess_letter)
                //     .cloned();
                //
                // if let Some(mut yellow_letter) = already_identified_as_yellow {
                //     //decrement the count of the yellow character, as one of them has been found
                //     println!("{:?}", &yellow_letter);
                //
                //     yellow_letter.count -= 1;
                //     if yellow_letter.count == 0 {
                //         //remove the yellow character from the hashset as all of them have been found
                //         yellow_characters.retain(|x| x.letter != guess_letter);
                //     } else {
                //         yellow_characters.replace(yellow_letter);
                //     }
                // }
            }
            'Y' => {
                let in_hashset_already: Option<YellowCharacter> = yellow_characters
                    .iter()
                    .find(|x| x.letter == guess_letter)
                    .cloned();

                if let Some(mut yellow_character) = in_hashset_already {
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

                    // println!("{}", position as u8);
                    yellow_character.not_positions.insert(position as u8);
                    yellow_characters.replace(yellow_character.clone());
                    // println!("{:?}", &yellow_character.not_positions);
                    // println!("{:?}", &yellow_characters);
                } else {
                    yellow_characters.insert(YellowCharacter {
                        letter: guess_letter,
                        not_positions: vec![position as u8].into_iter().collect::<HashSet<u8>>(),
                        count: 1,
                    });
                }
            }
            'g' => {
                //TODO: this aint right...
                let amount_of_greys_of_letter_already_stored = &grey_letters
                    .clone()
                    .into_iter()
                    .filter(|x| x.letter.chars().nth(0) == Some(guess_letter))
                    .count();

                let amount_of_grey_of_letter_in_word = colors
                    .chars()
                    .enumerate()
                    .filter(|&(index, element)| {
                        element == 'g' && &guess.word.chars().nth(index) == &Some(guess_letter)
                    })
                    .count();

                let amount_of_additional_grey_letters_to_store =
                    amount_of_grey_of_letter_in_word - amount_of_greys_of_letter_already_stored;

                if amount_of_additional_grey_letters_to_store > 0 {
                    for a in 0..amount_of_additional_grey_letters_to_store {
                        let code = a + amount_of_greys_of_letter_already_stored;
                        grey_letters.insert(GreyLetter {
                            letter: guess_letter.to_string() + &code.to_string(),
                        });
                    }
                }
            }
            _ => {
                panic!("Invalid color");
            }
        }
    }
}

//TODO: write tests
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

        if valid_answer_words.len() <= 10 {
            println!("Answer words = {:?}", valid_answer_words);
        }

        println!("Guess = {}", guess.word);

        if &guess == answer {
            println!("Answer = {}, in {} tries", guess.word, turn_count);
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

        println!(
            "Greens = {:?}, Yellows = {:?}, Greys = {:?}",
            green_letters, yellow_characters, grey_letters
        );

        // println!("green letters = {:?}", green_letters);
        // println!("yellow characters = {:?}", yellow_characters);
        // println!("grey letters = {:?}", grey_letters);

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

    println!("Answer words = {:?}", valid_answer_words);

    println!(
        r#"

             Fail! Answer = {}

             "#,
        answer.word
    );

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

    macro_rules! test_store_colors {
        ($(
            $function_name: ident,
            $guess: expr,
            $colors: expr,
            [$(($green_letter_before_letter: expr, $green_letter_before_position: expr)), *],
            [$(($yellow_character_before_letter: expr, $yellow_character_before_count: expr, {$($before_not_position: expr), *})), *],
            [$($grey_letter_before_letter: expr), *],
            [$(($green_letter_after_letter: expr, $green_letter_after_position: expr)), *],
            [$(($yellow_character_after_letter: expr, $yellow_character_after_count: expr, {$($after_not_position: expr), *})), *],
            [$($grey_letter_after_letter: expr), *]
        ), *) => {
            $ (
                #[test]
                fn $function_name() {
                    let guess = Word::new($guess.to_string());
                    let mut green_letters_before: HashSet<GreenLetter> = HashSet::new();
                    $(
                        green_letters_before.insert(GreenLetter {
                            letter: $green_letter_before_letter,
                            position: $green_letter_before_position
                        });
                    )*

                    let mut yellow_characters_before: HashSet<YellowCharacter> = HashSet::new();
                    $(
                        let mut yellow_character_before = YellowCharacter {
                            letter: $yellow_character_before_letter,
                            count: $yellow_character_before_count,
                            not_positions: HashSet::new()
                        };
                        $(
                            yellow_character_before.not_positions.insert($before_not_position);
                        )*
                        yellow_characters_before.insert(yellow_character_before);
                    )*

                    let mut grey_letters_before: HashSet<GreyLetter> = HashSet::new();

                    $(
                        grey_letters_before.insert(GreyLetter {
                            letter: $grey_letter_before_letter.to_string()
                        });
                    )*

                    let mut green_letters_after: HashSet<GreenLetter> = HashSet::new();

                    $(
                        green_letters_after.insert(GreenLetter {
                            letter: $green_letter_after_letter,
                            position: $green_letter_after_position
                        });
                    )*

                    let mut yellow_characters_after: HashSet<YellowCharacter> = HashSet::new();

                    $(
                        let mut yellow_character_after = YellowCharacter {
                            letter: $yellow_character_after_letter,
                            count: $yellow_character_after_count,
                            not_positions: HashSet::new()
                        };
                        $(
                            yellow_character_after.not_positions.insert($after_not_position);
                        )*
                        yellow_characters_after.insert(yellow_character_after);
                    )*

                    let mut grey_letters_after: HashSet<GreyLetter> = HashSet::new();

                    $(
                        grey_letters_after.insert(GreyLetter {
                            letter: $grey_letter_after_letter.to_string()
                        });
                    )*

                    store_colors(&guess, $colors, &mut green_letters_before, &mut yellow_characters_before, &mut grey_letters_before);

                    assert_eq!(green_letters_before, green_letters_after);
                    assert_eq!(yellow_characters_before, yellow_characters_after);
                    assert_eq!(grey_letters_before, grey_letters_after);
                }
            )*
        };
    }

    //TODO: more tests
    test_store_colors!(
        test_store_colors_1,
        "hello",
        "ggYGY", // if the answer is 'world'
        [],
        [],
        [],
        [('l', 3)],
        [('l', 1, { 2 }), ('o', 1, { 4 })],
        ["h0", "e0"],
        test_store_colors_2,
        "hello",
        "ggYGY",
        [('l', 3)],
        [('l', 1, { 2 }), ('o', 1, { 4 })],
        ["h0", "e0"],
        [('l', 3)],
        [('l', 1, { 2 }), ('o', 1, { 4 })],
        ["h0", "e0"],
        test_store_colors_3,
        "lords",
        "YGGgg",
        [],
        [('l', 1, { 2 }), ('o', 1, { 4 })],
        ["h0", "e0"],
        [('o', 1), ('r', 2)],
        [('l', 1, { 0,2 }), ('o', 1, {4})],
        ["h0", "e0", "d0", "s0"],
        test_store_colors_4,
        "world",
        "GGGGG",
        [],
        [('l', 1, { 2 }), ('o', 1, { 4 })],
        ["h0", "e0"],
        [('w', 0), ('o', 1), ('r', 2), ('l', 3), ('d', 4)],
        [('o', 0, {4}), ('l', 0, {2})],
        ["h0", "e0"]
    );
}
