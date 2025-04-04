use std::collections::{ HashMap };

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut char_map: HashMap<char, usize> = HashMap::new();

        for s_item in s.chars() {
            *char_map.entry(s_item).or_insert(0) += 1;
        }

        if s.len() % 2 == 0 {
            return char_map.iter().all(|(k, v)| v % 2 == 0);
        }
        return char_map.iter().filter(|(k, v)| **v % 2 != 0).count() == 1;
    }
}