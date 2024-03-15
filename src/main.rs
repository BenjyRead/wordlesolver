use std::fs;
// use std::time::Instant;

mod wordle_game;
mod distribution;
mod filters;

use wordle_game::play_game_of_wordle;


fn main() {

    let words_string = fs::read_to_string("src/valid_answer_words.txt").expect("Error reading file");
    let word_vector: Vec<&str> = words_string.lines().collect();

    let mut average_guesses = 0;
    let mut failed_games= 0;
    for a in 0..word_vector.len(){
        let result = play_game_of_wordle(a as u32);
        match result {
            Some(guesses) => average_guesses += guesses as u128,
            None => failed_games += 1,
        }
    }
    average_guesses = average_guesses/(word_vector.len() as u128);

    println!("Average guesses: {}", average_guesses);
    println!("Failed games: {}", failed_games);
}
