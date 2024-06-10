mod filtering;
mod word_processing;
mod word_structs;
mod wordle_game_simulation;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    for answer in &valid_answer_words {
        simulate_game(answer, &valid_guess_words, &valid_answer_words);
    }
}
