impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);
        let mut prime_nums: [bool; 32] = [true; 32];
        let mut result = 0;

        prime_nums[0] = false;
        prime_nums[1] = false;

        for id in 2 .. 32 {
            if !prime_nums[id] {
                continue;
            }

            for mul in 2 .. (32 / id) {
                prime_nums[id * mul] = false;
            }
        }

        for num in left ..= right {
            result += prime_nums[num.count_ones() as usize] as i32;
        }

        result
    }
}