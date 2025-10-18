use std::collections::HashSet;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut dist_count = 0;
        
        nums.sort();
        
        nums[0] -= k;

        let mut prev_num = nums[0];

        for id in 1 .. nums.len() {
            let original =  nums[id];

            nums[id] = nums[id].max(nums[id - 1]);
            let prev_num = nums[id - 1];
            let min_range = k.min(nums[id] - prev_num - 1);

            if min_range > 0 {
                nums[id] -= min_range;
                continue;
            } else if min_range == 0 {
                continue;
            }

            println!("{id} {k} {prev_num} {} {}", nums[id], original);

            if k >= (nums[id] - original + 1) {
                nums[id] += 1;
            }
        }

        println!("{nums:?}");

        let nums: HashSet<i32> = nums.into_iter().collect();

        nums.len() as i32
    }
}