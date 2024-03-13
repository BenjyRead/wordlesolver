use std::fs;

fn main() {
    let wordle_string_result = fs::read_to_string("src/words.txt");

    match wordle_string_result {
        Ok(ref wordle_string_result ) => wordle_string_result,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let wordle_string = wordle_string_result.unwrap();

    let word_vector: Vec<&str> = wordle_string.split("\n").collect();

}
