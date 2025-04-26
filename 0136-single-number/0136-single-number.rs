impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for &num_item in nums.iter() {
            res ^= num_item;
        }
        res
    }
}