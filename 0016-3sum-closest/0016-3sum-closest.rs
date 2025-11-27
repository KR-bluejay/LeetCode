impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut closest_diff = i32::MIN / 2;

        nums.sort_unstable();
        
        for mid_id in 0 .. nums.len() - 2 {
            let mut left_id = mid_id + 1;
            let mut right_id = nums.len() - 1;

            while left_id < right_id {
                let cur_sum = nums[left_id] + nums[mid_id] + nums[right_id];

                if (target - cur_sum).abs() < (target - closest_diff).abs() {
                    closest_diff = cur_sum;
                }

                if cur_sum < target {
                    left_id += 1
                } else {
                    right_id -= 1;
                }
            }
        }

        closest_diff
    }
}