use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut min_dist = u32::MAX;

        let mut num_map: HashMap<i32, u32> = HashMap::with_capacity(nums.len());

        for (id, num) in nums.into_iter().enumerate() {
            let id = id as u32;

            let mut tmp = num;
            let mut rev_num = 0;

            while tmp > 0 {
                rev_num = rev_num * 10 + tmp % 10;
                tmp /= 10;
            }

            if let Some(&diff_id) = num_map.get(&num) {
                min_dist = min_dist.min(id - diff_id);
            }

            num_map.insert(rev_num, id);
        }

        min_dist as i32
    }
}