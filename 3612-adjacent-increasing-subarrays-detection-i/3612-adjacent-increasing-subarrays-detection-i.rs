impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut sub_count = 1;

        for i in 0 .. (nums.len() - k) {
            if nums[i] < nums[i + 1] && nums[i + k] < *nums.get(i + k + 1).unwrap_or(&i32::MIN) {
                sub_count += 1;
            } else {
                sub_count = 1;
            }

            if sub_count >= k {
                return true;
            }
        }

        false
    }
}