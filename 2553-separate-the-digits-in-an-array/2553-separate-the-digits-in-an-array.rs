impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len() * 6);

        for mut num in nums.into_iter() {
            let num_len = num.ilog10() as usize + 1;

            let start_id = result.len();
            let end_id = result.len() + num_len;

            unsafe {
                result.set_len(result.len() + num_len);
            }

            for id in (start_id .. end_id).rev() {
                result[id] = num % 10;
                num /= 10;
            }
        }

        result
    }
}