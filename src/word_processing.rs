use std::collections::{HashMap, HashSet};

fn string_counter(word: &str) -> HashMap<char, u32> {
    return word.chars().fold(HashMap::new(), |mut counter, c| {
        *counter.entry(c).or_insert(0) += 1;
        return counter
    });
}

pub fn get_letter_set(word: &str) -> HashSet<String> {
    let counter = string_counter(word);

    let mut letter_set = HashSet::new();

    for (letter, count) in counter {
        for a in 0..count {
            letter_set.insert(format!("{}{}", letter, a));
        }
    }

    return letter_set;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_counter() {
        let test_string_counter_test_cases = vec![
            (
                "hello",
                [('h', 1), ('e', 1), ('l', 2), ('o', 1)] // generate a hashmap
                    .into_iter()
                    .collect(),
            ),
            (
                "world",
                [('w', 1), ('o', 1), ('r', 1), ('l', 1), ('d', 1)]
                    .into_iter()
                    .collect(),
            ),
        ];

        for (word, hashmap) in test_string_counter_test_cases {
            assert_eq!(string_counter(word), hashmap);
        }
    }

    #[test]
    fn test_get_letter_set() {
        let test_get_letter_set_test_cases = vec![
            (
                "hello",
                HashSet::from_iter(
                    vec!["h0", "e0", "l0", "l1", "o0"].into_iter().map(String::from),
                ),
            ),
            (
                "world",
                HashSet::from_iter(
                    vec!["w0", "o0", "r0", "l0", "d0"]
                    .into_iter()
                    .map(String::from),
                ),
            ),
            (
                "lllll",
                HashSet::from_iter(
                    vec!["l0", "l1", "l2", "l3", "l4"]
                    .into_iter()
                    .map(String::from),
                ),
            ),
        ];

        for (word, hashset) in test_get_letter_set_test_cases {
            assert_eq!(get_letter_set(word), hashset);
        }
    }
}
