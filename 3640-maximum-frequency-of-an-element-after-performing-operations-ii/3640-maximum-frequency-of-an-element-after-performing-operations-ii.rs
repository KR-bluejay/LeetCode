impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort();

        let mut num_id: usize = 0;

        let mut num_counts: Vec<i32> = Vec::with_capacity(nums.len() / 2);
        let mut num_values: Vec<i32> = Vec::with_capacity(nums.len() / 2);
        let mut max_freq_count = 0;

        while num_id < nums.len() {
            let num_value = nums[num_id];
            
            let mut num_count = 1;
            let mut next_num_id = num_id + 1;

            while next_num_id < nums.len() && num_value == nums[next_num_id] {
                num_count += 1;
                next_num_id += 1;
            }

            num_counts.push(num_count);
            num_values.push(num_value);

            num_id = next_num_id;
        }
        
        let mut base_id = 0;

        for (num_id, num_value) in nums.iter().enumerate() {
            while base_id < nums.len() && 2 * k < num_value - nums[base_id] {
                base_id += 1;
            }

            max_freq_count = max_freq_count.max((num_id as i32 - base_id as i32 + 1)
                .min(num_operations));
        }

        let mut left_id = 0;
        let mut right_id = 0;


        for (num_id, num_value) in num_values.iter().enumerate() {
            while left_id < nums.len() && k < num_value - nums[left_id] {
                left_id += 1;
            }

            while right_id < nums.len() && nums[right_id] - num_value <= k {
                right_id += 1;
            }

            let cur_count = num_counts[num_id] as i32;
            let add_count = ((right_id as i32 - left_id as i32) - cur_count).min(num_operations);

            max_freq_count = max_freq_count
                .max(cur_count + add_count);
        }

        max_freq_count
    }
}