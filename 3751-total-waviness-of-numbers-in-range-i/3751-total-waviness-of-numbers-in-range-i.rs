impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut result = 0;

        for mut num in num1.max(101) ..= num2 {
            let mut right = num % 10;
            num /= 10;
            let mut mid = num % 10;
            num /= 10;


            while num > 0 {
                let left = num % 10;
                num /= 10;

                let is_peak = left < mid && mid > right;
                let is_valley = left > mid && mid < right;

                result += (is_peak || is_valley) as i32;

                right = mid;
                mid = left;
            }
        }

        result
    }
}