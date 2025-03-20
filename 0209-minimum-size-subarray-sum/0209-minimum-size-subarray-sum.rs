impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left_index: usize = 0;
        let mut cur_sum: i32 = 0;
        
        let mut cur_count = 0;
        let mut min_count = usize::MAX;

        for right_index in 0 .. nums.len() {
            cur_sum += nums[right_index];

            while target <= cur_sum && left_index <= right_index {
                cur_count = right_index - left_index + 1;
                min_count = min_count.min(cur_count);

                cur_sum -= nums[left_index];
                left_index += 1;
            }
        }

        if min_count == usize::MAX {
            return 0
        }

        return min_count as i32;
    }
}