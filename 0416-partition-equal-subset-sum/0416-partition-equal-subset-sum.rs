use std::collections::{HashSet};
impl Solution {
    fn dp(num_id: usize, nums: &Vec<i32>, s1: i32, s2: i32, num_sum: i32, visited: &mut HashSet<(i32, i32)>) -> bool {
        if num_id == nums.len() {
            return s1 == s2;
        }

        if visited.contains(&(s1, s2)) {
            return false;
        }


        let cur_num = nums[num_id];

        if s1 + cur_num == (num_sum / 2) || s2 + cur_num == (num_sum / 2)  {
            return true;
        }

        if s1 + cur_num > (num_sum / 2) || s2 + cur_num > (num_sum / 2) {
            visited.insert((s1, s2));
            return false;
        }


        if s1 + cur_num < (num_sum / 2) {
            if Self::dp(num_id + 1, nums, s1 + cur_num, s2, num_sum, visited) {
                return true;
            } else {
                visited.insert((s1 + cur_num, s2));
            }
        }
        if s2 + cur_num < (num_sum / 2) {
            if Self::dp(num_id + 1, nums, s1, s2 + cur_num, num_sum, visited) {
                return true;
            } else {
                visited.insert((s1, s2 + cur_num));
            }
        }
        false

    }
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        nums.sort();
        let num_sum = nums.iter().sum();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        if num_sum % 2 == 0 {
            return Self::dp(0, &nums, 0, 0, num_sum, &mut visited);
        }
        false
    }
}