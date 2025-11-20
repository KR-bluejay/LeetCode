impl Solution {
    fn dp(count: i32, id: usize, nums: &Vec<i32>, jump_counts: &mut Vec<i32>) {
        if id >= nums.len() {
            jump_counts[nums.len() - 1] = jump_counts[nums.len() - 1].min(count);

            return;
        }

        if jump_counts[id] < count || jump_counts[nums.len() - 1] <= count {
            return;
        }

        jump_counts[id] = count;

        let mut max_id = id;
        

        for next_id in (id + 1 ..= id + nums[id] as usize).rev() {
            let next_count = count + 1;

            if next_id >= nums.len() {
                jump_counts[nums.len() - 1] = jump_counts[nums.len() - 1].min(next_count);

                continue;
            }


            jump_counts[next_id] = jump_counts[next_id].min(next_count);

            if max_id == id {
                max_id = next_id;
            } else if (nums[max_id] + (max_id - next_id) as i32) < nums[next_id] {
                max_id = next_id;
            }
        }

        Self::dp(count + 1, max_id, nums, jump_counts);
    }
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jump_counts: Vec<i32> = vec![i32::MAX; nums.len()];

        Self::dp(0, 0, &nums, &mut jump_counts);

        *jump_counts.last().unwrap()
    }
}