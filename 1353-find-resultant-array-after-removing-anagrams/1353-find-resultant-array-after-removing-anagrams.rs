use itertools::Itertools;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut word_stack: Vec<(String, String)> = Vec::with_capacity(words.len());

        for word in words {
            let sorted_word = word.bytes().sorted().map(|b| b as char).collect();

            if let Some((_, sorted_in_stack)) = word_stack.last() {
                if *sorted_in_stack == sorted_word {
                    continue;
                }
            }
            word_stack.push((word.to_string(), sorted_word));
        }

        word_stack.into_iter().map(|v| v.0).collect()
    }
}