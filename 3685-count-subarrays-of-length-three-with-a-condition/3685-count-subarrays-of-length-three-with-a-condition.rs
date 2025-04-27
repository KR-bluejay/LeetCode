impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        
        for i in 0 .. nums.len() - 2 {
            if nums[i] as f32 + nums[i + 2] as f32 == nums[i + 1] as f32 / 2 as f32 {
                result += 1;
            }
        }

        result
    }
}