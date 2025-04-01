use std::cmp;

impl Solution {
    fn dp(num_id: usize, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if num_id == nums.len() {
            return 0;
        }

        if cache[num_id] != -1 {
            return cache[num_id];
        }

        // let skip = Self::dp(num_id + 1, nums, cache);
        // let skip = nums[num_id];
        let skip = nums[num_id];
        let contain = nums[num_id] + Self::dp(num_id + 1, nums, cache);

        cache[num_id] = cmp::max(skip, contain);
        cache[num_id]
    }
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cache: Vec<i32> = vec![-1; nums.len()];
        let mut max_val = i32::MIN;

        Self::dp(0, &nums, &mut cache);

        for &cache_item in cache.iter() {
            max_val = max_val.max(cache_item);
        }
        max_val
    }
}