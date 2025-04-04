use std::collections::{ HashMap };
use itertools::Itertools;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let char_freq_map: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        char_freq_map.values().filter(|&v| v % 2 != 0).count() <= 1
    }
}