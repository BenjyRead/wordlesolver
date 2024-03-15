use crate::distribution::suggest_word;
use std::fs;
use std::collections::HashSet;

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

fn calculate_yellow_letters(guess: &str, answer: &str, green_letters: &str) -> HashSet<char> {
    let mut yellow_letters = HashSet::new();
    for (a, letter) in guess.chars().enumerate() {

        if green_letters.chars().nth(a).expect("No such index in string") != '_' {
            continue;
        }

        if answer.contains(letter) {
            yellow_letters.insert(letter);
        }

    }
    return yellow_letters;
}

fn calculate_grey_letters(guess: &str, answer: &str) -> HashSet<char> {
    let mut grey_letters = HashSet::new();
    for letter in guess.chars() {
        if !answer.contains(letter) {
            grey_letters.insert(letter);
        }
    }
    return grey_letters;
}

// fn remove_duplicates_from_string(string: &str) -> String {
//     return string.chars().collect::<HashSet<char>>().iter().collect();
// }

fn remove_green_letters_from_set(set: &HashSet<char>, green_letters: &str) -> HashSet<char> {
    let mut new_set = set.clone();
    for letter in green_letters.chars(){
        new_set.remove(&letter);
    }
    return new_set;
}

pub fn play_game_of_wordle(answer_seed: u32) -> Option<u8>{
    let guess_words_string = fs::read_to_string("src/valid_guess_words.txt").expect("Error reading file");
    let guess_word_vector: Vec<&str> = guess_words_string.lines().collect();

    let answers_string = fs::read_to_string("src/valid_answer_words.txt").expect("Error reading file");
    let answer_word_vector: Vec<&str> = answers_string.lines().collect();

    let answer = answer_word_vector[answer_seed as usize];
    println!("Answer: {}", answer);

    let mut green_str =  String::new();
    let mut yellow_set = HashSet::new();
    let mut grey_set = HashSet::new();
    let mut guess = String::new();
    let mut guess_count: u8 = 0;
    // println!("Suggested word: {}", suggest_word(green_str, yellow_str, grey_str, word_vector));
    while guess != answer{

        if guess_count == 7 {
            println!("---Loss!---");
            return None;
        }

        guess = suggest_word(&green_str, &yellow_set, &grey_set, guess_word_vector.clone());
        guess_count += 1;
        green_str = calculate_green_letters(&guess, &answer);
        yellow_set.extend(&calculate_yellow_letters(&guess, &answer, &green_str));
        grey_set.extend(&calculate_grey_letters(&guess, &answer));

        yellow_set = remove_green_letters_from_set(&yellow_set, &green_str);
        grey_set = remove_green_letters_from_set(&grey_set, &green_str);

        println!("Guess: {}, Green Letters {}, Yellow Letters {:?}, Grey Letters {:?}", guess, green_str, yellow_set, grey_set);
    }
    println!("---Win!---");
    return Some(guess_count);
}