impl Solution {
    pub fn num_of_subarrays(nums: Vec<i32>) -> i32 {
        const MODULO: i32 = 1_000_000_007;

        let mut result = 0;

        let mut odd_count = 0;
        let mut even_count = 0;

        let mut prefix = 0;

        for num in nums {
            prefix += num;

            if prefix % 2 == 1 {
                odd_count += 1;
                result += even_count + 1;
            } else {
                even_count += 1;
                result += odd_count;
            }
            result %= MODULO;
        }

        result
    }
}