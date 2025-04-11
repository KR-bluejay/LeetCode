use std::str::FromStr;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut num_count = 0;

        for i in low ..= high {
            let i_str: Vec<u32> = i.to_string().chars().map(|v| v.to_digit(10).unwrap()).collect();
            let i_str_len = i_str.len();

            if i_str_len % 2 != 0 {
                continue;
            }
            let mut l_value = 0;
            let mut r_value = 0;

            for (i, &v) in i_str.iter().enumerate() {
                if i < i_str_len / 2 {
                    l_value+= v;
                } else {
                    r_value+= v;
                }
            }

            if l_value == r_value {
                num_count += 1;
            }
        }
        num_count
    }
}