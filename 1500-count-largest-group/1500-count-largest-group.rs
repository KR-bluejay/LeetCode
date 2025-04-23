use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        if n < 10 {
            return n;
        }

        let mut num_map: HashMap<i32, Vec<i32>> = HashMap::new();
        num_map.insert(0, vec![]);
        for i in 1 ..= 9 {
            num_map.insert(i, vec![i]);
        }
        let mut prev_num = 0;

        for i in 10 ..= n {
            if i % 10 == 0 {
                prev_num = i.to_string().chars().map(|v| v.to_digit(10).unwrap() as i32).sum();
            }
            num_map.entry(prev_num).and_modify(|v| v.push(i)).or_insert(vec![i]);
            prev_num = prev_num + 1;
        }

        let mut max_len = 0;
        let mut max_count = 0;
        let mut all_elem = 0;


        for (num_key, num_list) in num_map.iter() {
            all_elem += num_list.len();
            if max_len < num_list.len() {
                max_count = 1;
                max_len = num_list.len();
            } else if max_len == num_list.len() {
                max_count += 1;
            }
        }

        println!("{all_elem}");

        // println!("{max_len}");

        max_count as i32
    }
}