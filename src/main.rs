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

fn main() {
    let start = Instant::now();

    let wordle_string = fs::read_to_string("src/words.txt").expect("ReadFileError: Could not read file");
    let word_vector: Vec<&str> = wordle_string.split("\n").collect();

    println!("Time to load file into array: {:?}", start.elapsed());

    let green_str = "eag__";
    let yellow_str = "l";
    let grey_str = "y";

    let overall = Instant::now();

    let valid_words_green = narrow_down_greens(green_str, word_vector);
    println!("Time elapsed to calculate green words: {:?}", overall.elapsed());

    let valid_words_yellow = narrow_down_yellows(yellow_str, valid_words_green);
    println!("Time elapsed to calculate yellow words: {:?}", overall.elapsed());

    let narrow_down_greys = narrow_down_greys(grey_str, valid_words_yellow);

    println!("{:?}", narrow_down_greys);
    println!("Time elapsed to calculate valid words: {:?}", overall.elapsed());
}
