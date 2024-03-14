use std::collections::HashMap;

fn string_counter(word: &str) -> HashMap<char, u8> {
    // Attempts to operate similar to how Counter() does in python

    let mut counter = HashMap::new();
    for char in word.chars() {

        // or_insert returns a mutable reference to the value
        // which is why I need to dereference it to add one
        *counter.entry(char).or_insert(0) += 1;
    }
    return counter;
}

pub fn find_letter_distribution(word_vector: Vec<&str>) -> HashMap<String, u32> {
    // This function counts the number of times each letter appears in the word_vector
    // but also counts duplicates as separate letters. This way we can assign points
    // to each letter based on how common it is in the word_vector, and use that to
    // suggest the best word to the user

    let mut distribution: HashMap<String, u32> = HashMap::new();
    for word in word_vector {
        let char_counter = string_counter(word);

        for (char, count) in char_counter {

            for a in 1..count+1{
                let letter_code = a.to_string()+&char.to_string();
                *distribution.entry(letter_code).or_insert(0) += 1;
            }

        }

    }
    return distribution;
}

fn get_word_points(word: &str, letter_distribution: &HashMap<String, u32>) -> u32 {
    let mut points = 0;
    let letter_counter = string_counter(word);
    // There will be no value more than 1 in the hashmap
    for (char, count) in letter_counter {

        for a in 1..count+1{

            let letter_code = a.to_string()+&char.to_string();
            points += letter_distribution.get(&letter_code).expect("No such key in hashmap");
        }

    }
    return points;
}

pub fn get_highest_point_word(word_vector: Vec<&str>, letter_distribution: &HashMap<String, u32>) -> String {
    let mut highest_points = 0;
    let mut highest_word = "";
    for word in word_vector {
        let points = get_word_points(word, letter_distribution);
        if points > highest_points {
            highest_points = points;
            highest_word = word;
        }
    }
    return highest_word.to_string();
}