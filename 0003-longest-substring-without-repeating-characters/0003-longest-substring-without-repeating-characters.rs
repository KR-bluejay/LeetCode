use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut max_len = 0;
        let mut word_map: HashMap<char, usize> = HashMap::new();

        let mut s_list: Vec<char> = s.chars().collect();


        for (char_index, char_item) in s_list.iter().enumerate() {
            if let Some(old_index) = word_map.insert(*char_item, char_index) {
                left = left.max(old_index + 1);
            }

            let cur_len = char_index - left + 1;

            max_len = max_len.max(cur_len);
        }

        max_len as i32
    }
}