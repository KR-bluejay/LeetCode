use std::collections::{ HashMap };

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut num_freq_map_a: HashMap<i32, usize> = HashMap::new();
        let mut num_freq_map_b: HashMap<i32, usize> = HashMap::new();


        for num_item in nums.iter() {
            *num_freq_map_a.entry(*num_item).or_insert(0) += 1;
        }

        let domi_num = (num_freq_map_a.iter().max_by_key(|entry| entry.1).unwrap().0).clone();

        for i in 0 .. nums.len() {
            let cur_num = nums[i];

            *num_freq_map_a.entry(cur_num).or_insert(0) -= 1;
            *num_freq_map_b.entry(cur_num).or_insert(0) += 1;

            let b_len = i + 1;
            let a_len = nums.len() - b_len;

            let a_count = num_freq_map_a.get(&domi_num).unwrap_or(&0);
            let b_count = num_freq_map_b.get(&domi_num).unwrap_or(&0);


            if *a_count > 0 && *b_count > 0 && a_len / a_count  < 2 && b_len / b_count < 2  {
                return i as i32
            }
        }


        -1
    }
}