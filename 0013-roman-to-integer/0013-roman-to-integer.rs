use std::collections::BTreeMap;

impl Solution {
    pub fn roman_to_int(mut s: String) -> i32 {
        let roman_num_map = vec![
            ("IV", " 4 "),
            ("IX", " 9 "),
            ("XL", " 40 "),
            ("XC", " 90 "),
            ("CD", " 400 "),
            ("CM", " 900 "),
            ("I", " 1 "),
            ("V", " 5 "),
            ("X", " 10 "),
            ("L", " 50 "),
            ("C", " 100 "),
            ("D", " 500 "),
            ("M", " 1000 ")
        ];

        for (key, value) in roman_num_map {
            s = s.replace(key, value);
        }

        let sum = s.split_whitespace()
                .map(|item| item.parse::<i32>().expect("A"))
                .fold(0, |acc, item| acc + item);
        sum
    }
}