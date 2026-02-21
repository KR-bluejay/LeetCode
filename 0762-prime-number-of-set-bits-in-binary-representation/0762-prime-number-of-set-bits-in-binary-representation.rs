impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);
        let mut prime_nums: Vec<bool> = vec![true; right + 1];
        let mut result = 0;

        prime_nums[0] = false;
        prime_nums[1] = false;

        for id in 2 ..= right {
            if !prime_nums[id] {
                continue;
            }

            for mul in 2 ..= (right / id) {
                prime_nums[id * mul] = false;
            }
        }

        for num in left ..= right {
            result += prime_nums[num.count_ones() as usize] as i32;
        }

        result
    }
}