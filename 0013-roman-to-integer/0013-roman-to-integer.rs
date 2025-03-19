use std::collections::HashMap;


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_num_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut result_num = 0;
        let s_chars: Vec<char> = s.chars().collect();

        for (roman_index, roman_item) in s_chars.iter().enumerate() {
            let cur_num = roman_num_map.get(&roman_item).unwrap();

            if (roman_index + 1 < s.len() 
                && cur_num < roman_num_map.get(&s_chars[roman_index + 1]).unwrap()) {
                result_num -= cur_num;

                continue;
            }

            result_num += cur_num
        }
        result_num
    }
}