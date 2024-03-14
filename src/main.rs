use std::fs;
use std::time::Instant;
use std::collections::HashSet;

mod filters;
// use filters::{narrow_down_greens,narrow_down_greys,narrow_down_yellows};
use filters::get_valid_words;

mod distribution;
use distribution::{find_letter_distribution, get_highest_point_word};



fn word_has_duplicate_characters(word: &str) -> bool {
    let mut set = HashSet::new();
    for char in word.chars(){
        if !set.insert(char){
            return true;
        }
    }
    return false;
}

fn filter_duplicates(word_vector: Vec<&str>) -> Vec<&str> {
    // This function title is kinda vague, but I'm not sure what to call it
    // It removes all duplicate words from the vector, in the case there are
    // only duplicates, it returns the original vector
    let mut copy_vector = word_vector.clone();
    // This is weird code, set.insert both inserts
    // and returns a bool if it went through ok
    // better than raising ig

    copy_vector.retain(|&word| !word_has_duplicate_characters(word));

    if copy_vector.is_empty() {
        return word_vector;
    }
    return copy_vector;
}

fn suggest_word<'a>(green_str: &'a str, yellow_str: &'a str, grey_str: &'a str, word_vector: Vec<&'a str>) -> String {
    let valid_words = get_valid_words(green_str, yellow_str, grey_str, word_vector);
    let letter_distribution = find_letter_distribution(valid_words.clone());

    return get_highest_point_word(valid_words, &letter_distribution)
}

fn main() {

    let words_string = fs::read_to_string("src/words.txt").expect("Error reading file");
    let word_vector: Vec<&str> = words_string.lines().collect();

    let green_str = "______";
    let yellow_str = "";
    let grey_str = "";
    println!("Suggested word: {}", suggest_word(green_str, yellow_str, grey_str, word_vector));
}
