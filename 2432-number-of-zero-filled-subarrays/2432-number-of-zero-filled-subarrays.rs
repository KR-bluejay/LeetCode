impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut streak: i64 = 0;
        let mut total_count: i64 = 0;

        for num in nums {
            streak = if num == 0 {
                streak + 1
            } else {
                0
            };

            total_count += streak;
        }

        total_count
    }
}