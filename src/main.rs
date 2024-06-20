mod filtering;
mod word_processing;
mod word_structs;
mod wordle_game_simulation;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u128;

use word_structs::Word;
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

    // println!("{}", valid_guess_words.len());
    // println!("{}", valid_answer_words.len());

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
}
