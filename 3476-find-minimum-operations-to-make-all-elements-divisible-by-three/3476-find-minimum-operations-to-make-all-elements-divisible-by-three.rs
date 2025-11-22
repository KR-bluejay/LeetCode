impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums.into_iter() {
            if num % 3 == 0 {
                continue;
            } else {
                result += 1;
            }
        }

        result
    }
}