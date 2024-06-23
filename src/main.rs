mod filtering;
mod word_processing;
mod word_structs;
mod wordle_game_simulation;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use filtering::filter_words;
use word_structs::{GreenLetter, GreyLetter, Word, YellowCharacter};
use wordle_game_simulation::simulate_game;

fn read_lines_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    return lines;
}

fn main() {
    let valid_guess_words: HashSet<Word> = read_lines_from_file("assets/valid_guess_words.txt")
        .into_iter()
        .map(|word| Word::new(word))
        .collect();
    let valid_answer_words: HashSet<Word> = read_lines_from_file("assets/valid_answer_words.txt")
        .into_iter()
        .map(|word| Word::new(word))
        .collect();

    // // Define the data
    // let green_letters = vec![GreenLetter {
    //     letter: 'e',
    //     position: 4,
    // }];
    // let yellow_letters = vec![YellowCharacter {
    //     letter: 'n',
    //     not_positions: vec![1].into_iter().collect(),
    //     count: 1,
    // }];
    // let grey_letters = vec![
    //     GreyLetter {
    //         letter: "i0".to_string(),
    //     },
    //     GreyLetter {
    //         letter: "t0".to_string(),
    //     },
    //     GreyLetter {
    //         letter: "s0".to_string(),
    //     },
    //     GreyLetter {
    //         letter: "e0".to_string(),
    //     },
    //     GreyLetter {
    //         letter: "r0".to_string(),
    //     },
    //     GreyLetter {
    //         letter: "o0".to_string(),
    //     },
    //     GreyLetter {
    //         letter: "a0".to_string(),
    //     },
    // ];
    //
    // // Convert vectors to HashSet
    // let green_letters: HashSet<GreenLetter> = green_letters.into_iter().collect();
    // let yellow_letters: HashSet<YellowCharacter> = yellow_letters.into_iter().collect();
    // let grey_letters: HashSet<GreyLetter> = grey_letters.into_iter().collect();
    //
    // println!(
    //     "{:?}",
    //     filter_words(
    //         &valid_guess_words,
    //         &grey_letters,
    //         &green_letters,
    //         &yellow_letters
    //     )
    // );

    println!("{}", valid_guess_words.len());
    println!("{}", valid_answer_words.len());

    let x = valid_answer_words.len();
    let mut fails: u32 = 0;
    let mut count: u32 = 0;
    for answer in &valid_answer_words {
        let y = simulate_game(answer, &valid_guess_words, &valid_answer_words);

        if y != -1 {
            count += y as u32;
        } else {
            fails += 1;
        }
    }

    println!("Average number of guesses: {}", count as f64 / x as f64);
    println!("Number of fails: {}", fails);

    // simulate_game(
    //     &Word::new("coral".to_string()),
    //     &valid_guess_words,
    //     &valid_answer_words,
    // );
}
