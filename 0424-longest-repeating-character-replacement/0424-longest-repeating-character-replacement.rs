use std::collections::{ HashMap };

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;

        let mut left_id: usize = 0;
        let mut max_freq: usize = 0;
        let mut max_len: usize = 0;

        let mut char_map: HashMap<char, usize> = HashMap::with_capacity(26);

        for (right_id, &char_item) in s.iter().enumerate() {
            let cur_freq = *char_map.entry(char_item)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            max_freq = max_freq.max(cur_freq);

            let shrink_range = right_id - left_id + 1 - max_freq - k;

            if shrink_range < s.len() {
                for i in 0 .. shrink_range {
                    char_map.entry(s[left_id])
                        .and_modify(|v| *v -= 1);
                    left_id += 1;
                }
            }

            max_len = max_len.max(right_id - left_id + 1);
        }
        max_len as i32
    }
}