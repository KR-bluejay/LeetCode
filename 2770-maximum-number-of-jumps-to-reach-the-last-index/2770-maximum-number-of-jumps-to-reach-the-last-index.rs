impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let mut jumps = vec![-1; nums.len()];

        jumps[0] = 0;

        for left_id in 0 .. nums.len() - 1 {
            if jumps[left_id] == -1 {
                continue;
            }

            for right_id in left_id + 1 .. nums.len() {
                if (nums[right_id] - nums[left_id]).abs() <= target  {
                    jumps[right_id] = jumps[right_id].max(jumps[left_id] + 1);
                }
            }
        }

        *jumps.last().unwrap()
    }
}