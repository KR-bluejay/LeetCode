impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut center_id = 0;
        let mut result = i64::MIN;

        while center_id < nums.len() {
            let mut center_sum = nums[center_id] as i64;

            let (mut left_id, mut right_id) = (center_id, center_id);
            let (mut left_sum, mut right_sum) = (0, 0);
            let (mut max_left, mut max_right) = (i64::MIN, i64::MIN);


            while right_id + 1 < nums.len() && nums[right_id] > nums[right_id + 1] {
                center_sum += nums[right_id + 1] as i64;
                
                right_id += 1;
            }

            if center_id == right_id {
                center_id += 1;
                
                continue;
            }

            let center_end = right_id;


            while left_id > 0 && nums[left_id - 1] < nums[left_id] {
                left_sum += nums[left_id - 1] as i64;
                max_left = max_left.max(left_sum);

                left_id -= 1;
            }

            if left_id == center_id {
                center_id += 1;
                
                continue;
            }

            while right_id + 1 < nums.len() && nums[right_id] < nums[right_id + 1] {
                right_sum += nums[right_id + 1] as i64;
                max_right = max_right.max(right_sum);
                
                right_id += 1;
            }

            if center_end == right_id {
                center_id += 1;

                continue;
            }

            center_id = right_id;
            result = result.max(max_left + center_sum + max_right);
        }

        result
    }
}