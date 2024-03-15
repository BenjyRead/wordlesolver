use std::collections::HashSet;


fn narrow_down_greens<'a>(green_str: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for (a,char) in green_str.chars().enumerate(){
        if char == '_'{continue;}
        narrowed_vector.retain(|&element| element.chars().nth(a) == green_str.chars().nth(a));
    }
    return narrowed_vector;
}

fn narrow_down_yellows<'a>(yellow_letters: &'a HashSet<char>, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for char in yellow_letters{
        narrowed_vector.retain(|&element| element.contains(*char));
    }
    return narrowed_vector;
}

fn narrow_down_greys<'a>(grey_letters: &'a HashSet<char>, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for char in grey_letters{
        narrowed_vector.retain(|&element| !element.contains(*char));
    }
    return narrowed_vector;
}


pub fn get_valid_words<'a>(green_str: &'a str, yellow_letters: &'a HashSet<char>, grey_letters: &'a HashSet<char>, word_vector: Vec<&'a str>) -> Vec<&'a str> {
    let valid_words_green = narrow_down_greens(green_str, word_vector);
    let valid_words_yellow = narrow_down_yellows(yellow_letters, valid_words_green);
    let valid_words_grey = narrow_down_greys(grey_letters, valid_words_yellow);
    return valid_words_grey;
}