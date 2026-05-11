impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        for num in nums.into_iter() {
            for num_byte in num.to_string().into_bytes() {
                result.push((num_byte - b'0') as i32);
            }
        }

        result
    }
}