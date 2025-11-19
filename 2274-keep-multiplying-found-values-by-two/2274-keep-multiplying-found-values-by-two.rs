use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        let num_len = nums.len();
        let num_set: HashSet<i32> = nums.into_iter().fold(HashSet::with_capacity(num_len), |mut acc, num| {
            acc.insert(num);
            acc
        });

        while num_set.get(&original).is_some() {
            original *= 2;
        }

        original
    }
}