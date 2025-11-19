use std::collections::HashMap;
use itertools::Itertools;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        let num_map = nums.iter().counts();

        while num_map.get(&original).is_some() {
            original *= 2;
        }

        original
    }
}