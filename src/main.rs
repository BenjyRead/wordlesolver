use std::fs;
use std::time::Instant;

fn narrow_down_greens<'a>(green_str: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for (a,char) in green_str.chars().enumerate(){
        if char == '_'{continue;}
        narrowed_vector.retain(|&element| element.chars().nth(a) == green_str.chars().nth(a));
    }
    return narrowed_vector;
}

fn narrow_down_yellows<'a>(yellow_letters: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for char in yellow_letters.chars(){
        // if !word.contains(char){
        //     narrowed_vector.retain(|&x| x != word);
        // }
        narrowed_vector.retain(|&element| element.contains(char));
    }
    return narrowed_vector;
}

fn narrow_down_greys<'a>(grey_letters: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for char in grey_letters.chars(){
        // if word.contains(char){
        //     narrowed_vector.retain(|&x| x != word);
        // }
        narrowed_vector.retain(|&element| !element.contains(char));
    }
    return narrowed_vector;
}

fn get_valid_words<'a>(green_str: &'a str, yellow_str: &'a str, grey_str: &'a str, word_vector: Vec<&'a str>) -> Vec<&'a str> {
    let valid_words_green = narrow_down_greens(green_str, word_vector);
    let valid_words_yellow = narrow_down_yellows(yellow_str, valid_words_green);
    let valid_words_grey = narrow_down_greys(grey_str, valid_words_yellow);
    return valid_words_grey;
}
fn suggest_word<'a>(green_str: &'a str, yellow_str: &'a str, grey_str: &'a str, word_vector: Vec<&'a str>) -> &'a str {
    let valid_words = get_valid_words(green_str, yellow_str, grey_str, word_vector);
    return valid_words[0];
}

fn main() {

    let green_str = "eag__";
    let yellow_str = "l";
    let grey_str = "y";

    let words_string = fs::read_to_string("src/words.txt").expect("Error reading file");
    let word_vector = words_string.lines().collect();

    let start = Instant::now();
    let suggested_word = suggest_word(green_str, yellow_str, grey_str, word_vector);
    let duration = start.elapsed();
    println!("Suggested word: {}", suggested_word);
    println!("Time to get suggested word is: {:?}", duration);
}
