impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut center_id = 0;

        let mut result = i64::MIN;

        let mut max_left: Vec<i64> = vec![i64::MIN; nums.len()];
        let mut max_right: Vec<i64> = vec![i64::MIN; nums.len()];

        for id in 1 .. nums.len() {
            max_left[id] = if nums[id - 1] < nums[id] {
                max_left[id - 1].max(0) + nums[id - 1] as i64
            } else {
                i64::MIN
            };
        }

        for id in (0 .. nums.len() - 1).rev() {
            max_right[id] = if nums[id] < nums[id + 1] {
                max_right[id + 1].max(0) + nums[id + 1] as i64
            } else {
                i64::MIN
            };
        }


        while center_id < nums.len() {
            let (mut left_id, mut right_id) = (center_id, center_id);
            let mut center_sum = nums[center_id] as i64;

            while right_id + 1 < nums.len() && nums[right_id] > nums[right_id + 1] {
                center_sum += nums[right_id + 1] as i64;
                
                right_id += 1;
            }

            if left_id < right_id {
                println!("A");
                let left_sum = max_left[left_id];
                let right_sum = max_right[right_id];

                if left_sum != i64::MIN && right_sum != i64::MIN {
                    println!("{left_sum} {center_sum} {right_sum}");
                    result = result.max(left_sum + center_sum + right_sum);
                } else {
                    println!("{center_id} {left_sum} {right_sum}");
                }

                center_id = right_id;
            } else {
                center_id += 1;
            }
        }

        result
    }
}