use crate::filtering::filter_words;
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

// fn get_colors(answer: &Word, guess: &Word) -> String {
//     //5 character string
//     let mut colors = String::from("     ");
// }

pub fn simulate_game(
    answer: &Word,
    valid_guess_words: &HashSet<Word>,
    valid_answer_words: &HashSet<Word>,
) -> i8 {
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

        if &guess == answer {
            print!("Answer = {}, in {} tries", guess.word, turn_count);
            return turn_count.try_into().unwrap();
        }
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
        "GGGGG"
    );
}