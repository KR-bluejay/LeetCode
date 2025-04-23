use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        if n < 10 {
            return n;
        }

        let mut num_map: HashMap<i32, i32> = HashMap::new();
        let mut prev_num = 1;

        for i in 1 ..= n {
            if i % 10 == 0 {
                prev_num = i.to_string().chars().map(|v| v.to_digit(10).unwrap() as i32).sum();
            }
            num_map.entry(prev_num).and_modify(|v| *v += 1).or_insert(1);
            prev_num = prev_num + 1;
        }

        let mut max_len = 0;
        let mut max_count = 0;
        let mut all_elem = 0;


        for (num_key, &num_list) in num_map.iter() {
            if max_len < num_list {
                max_count = 1;
                max_len = num_list;
            } else if max_len == num_list {
                max_count += 1;
            }
        }


        max_count as i32
    }
}