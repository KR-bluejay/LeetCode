use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut char_map: HashMap<char, usize> = HashMap::new();

        let mut left_id: usize = 0;
        let mut right_id: usize = 0;
        let mut long_char_len: usize = 0;
        
        // for i in 0 .. s.len() {
        //     let temp = char_map
        //         .entry(s[i])
        //         .and_modify(|v| *v += 1)
        //         .or_insert(1);
        //     let char_len = i - left_id + 1;

        //     if char_len - *temp <= k as usize || i + 1 == s.len() {
        //         long_char_len = i + 1;
        //         right_id = i;
        //     } else {
        //         break;
        //     }
        // }
        let mut max_count = 0;
        

        for i in 0  .. s.len() {
            let temp_count = char_map
                .entry(s[i])
                .and_modify(|v| *v += 1)
                .or_insert(1);

            let mut char_len = i - left_id + 1;
            max_count = max_count.max(*temp_count);

            while i - left_id + 1 - max_count > k as usize && left_id < i {
                char_map
                    .entry(s[left_id])
                    .and_modify(|v| *v -= 1);
                left_id += 1;
            }

            char_len = i - left_id + 1;
            long_char_len = long_char_len.max(i - left_id + 1);

        }

        long_char_len as i32
    }
}