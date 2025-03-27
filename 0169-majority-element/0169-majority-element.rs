impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let mut candidate_element = nums[0];
        let mut candidate_count = 1;

        for &num_item in nums.iter().skip(1) {
            if candidate_element == num_item {
                candidate_count += 1;
                
                continue;
            }

            if candidate_count == 0 {
                candidate_element = num_item;
                candidate_count = 1;
                
                continue;
            }

            candidate_count -= 1;
        }

        candidate_element
    }
}