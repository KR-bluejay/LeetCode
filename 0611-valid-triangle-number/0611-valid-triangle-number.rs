use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut triangle_count = 0;

        for k in (2 .. nums.len()).rev() {
            let mut i = 0;
            let mut j = k - 1;

            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    triangle_count += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }


        triangle_count as i32
    }
}