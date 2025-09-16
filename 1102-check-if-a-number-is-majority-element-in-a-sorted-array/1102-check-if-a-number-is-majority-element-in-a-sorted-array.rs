impl Solution {
    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        nums.iter().filter(|v| **v == target).count() * 2 > nums.len()
    }
}