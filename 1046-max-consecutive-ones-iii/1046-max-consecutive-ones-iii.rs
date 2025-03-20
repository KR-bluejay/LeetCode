impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut zero_count = 0;
        let mut max_sum = 0;

        for right in 0 .. nums.len() {
            if nums[right] == 0{
                zero_count += 1;
            }

            while zero_count > k {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            let cur_sum = right - left + 1;

            max_sum = max_sum.max(cur_sum);
        }

        max_sum as i32
    }
}