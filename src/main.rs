use std::fs;
// use std::time::Instant;

mod wordle_game;
mod distribution;
mod filters;

use wordle_game::play_game_of_wordle;


fn main() {

    let words_string = fs::read_to_string("src/valid_guess_words.txt").expect("Error reading file");
    let word_vector: Vec<&str> = words_string.lines().collect();

    let x = play_game_of_wordle(0);
    match x {
        Some(y) => println!("You won in {} guesses!", y),
        None => println!("You lost!"),
    }

}
