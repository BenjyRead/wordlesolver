use crate::distribution::suggest_word;
use std::fs;

fn calculate_green_letters(guess: &str, answer: &str) -> String {
    let mut green_letters = String::new();
    for (a, letter) in guess.chars().enumerate() {
        if letter == answer.chars().nth(a).expect("No such index in string") {
            green_letters.push(letter);
        } else {
            green_letters.push('_');
        }
    }
    return green_letters;
}

fn calculate_yellow_letters(guess: &str, answer: &str, green_letters: &str) -> String {
    let mut yellow_letters = String::new();
    for (a, letter) in guess.chars().enumerate() {

        if green_letters.chars().nth(a).expect("No such index in string") != '_' {
            continue;
        }

        if answer.contains(letter) {
            yellow_letters.push(letter);
        }

    }
    return yellow_letters;
}

fn calculate_grey_letters(guess: &str, answer: &str) -> String {
    let mut grey_letters = String::new();
    for letter in guess.chars() {
        if !answer.contains(letter) {
            grey_letters.push(letter);
        }
    }
    return grey_letters;
}

pub fn play_game_of_wordle(answer_seed: u32) -> Option<u8>{
    let guess_words_string = fs::read_to_string("src/valid_guess_words.txt").expect("Error reading file");
    let guess_word_vector: Vec<&str> = guess_words_string.lines().collect();

    let answers_string = fs::read_to_string("src/valid_answer_words.txt").expect("Error reading file");
    let answer_word_vector: Vec<&str> = answers_string.lines().collect();

    let answer = answer_word_vector[answer_seed as usize];
    println!("Answer: {}", answer);

    let mut green_str =  String::new();
    let mut yellow_str = String::new();
    let mut grey_str = String::new();
    let mut guess = String::new();
    let mut guess_count: u8 = 0;
    // println!("Suggested word: {}", suggest_word(green_str, yellow_str, grey_str, word_vector));
    while guess != answer{

        if guess_count == 7 {
            return None;
        }

        guess = suggest_word(&green_str, &yellow_str, &grey_str, guess_word_vector.clone());
        guess_count += 1;
        green_str = calculate_green_letters(&guess, &answer);
        yellow_str.push_str(&calculate_yellow_letters(&guess, &answer, &green_str));
        grey_str.push_str(&calculate_grey_letters(&guess, &answer));
        println!("Guess: {}, Green Letters {}, Yellow Letters {}, Grey Letters {}", guess, green_str, yellow_str, grey_str);
    }
    return Some(guess_count);
}