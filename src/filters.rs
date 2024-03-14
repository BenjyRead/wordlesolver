
pub fn narrow_down_greens<'a>(green_str: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for (a,char) in green_str.chars().enumerate(){
        if char == '_'{continue;}
        narrowed_vector.retain(|&element| element.chars().nth(a) == green_str.chars().nth(a));
    }
    return narrowed_vector;
}

pub fn narrow_down_yellows<'a>(yellow_letters: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for char in yellow_letters.chars(){
        // if !word.contains(char){
        //     narrowed_vector.retain(|&x| x != word);
        // }
        narrowed_vector.retain(|&element| element.contains(char));
    }
    return narrowed_vector;
}

pub fn narrow_down_greys<'a>(grey_letters: &'a str, valid_words: Vec<&'a str>) -> Vec<&'a str> {
    let mut narrowed_vector: Vec<&str> = valid_words.clone();
    for char in grey_letters.chars(){
        // if word.contains(char){
        //     narrowed_vector.retain(|&x| x != word);
        // }
        narrowed_vector.retain(|&element| !element.contains(char));
    }
    return narrowed_vector;
}