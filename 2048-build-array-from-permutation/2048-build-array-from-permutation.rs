impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_result: Vec<i32> = vec![0; nums.len()];

        for (num_id, &num_item) in nums.iter().enumerate() {
            let num_item = num_item as usize;

            nums_result[num_id] = nums[num_item];
        }

        nums_result
    }
}