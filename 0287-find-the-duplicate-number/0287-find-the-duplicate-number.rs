use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut num_set = HashSet::with_capacity(nums.len());

        for num in nums.into_iter() {
            if !num_set.insert(num) {
                return num;
            }
        }

        0
    }
}