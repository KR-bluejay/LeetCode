use std::collections::HashMap;

impl Solution {
    pub fn sum_four_divisors(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut result = 0;
        let last_num = *nums.last().unwrap() as usize;

        let mut is_prime_nums: Vec<bool> = vec![true; last_num + 1];
        let mut num_sieve: Vec<i32> = Vec::with_capacity(nums.len());

        for i in 2 ..= last_num {
            if !is_prime_nums[i] {
                continue;
            }
            num_sieve.push(i as i32);

            for j in 2 ..= last_num / i {
                is_prime_nums[i * j] = false;
            }
        }

        for num in nums {
            let mut count = 1;
            let mut sum = 1;

            for &prime_num in num_sieve.iter() {
                if num % prime_num != 0 || num < prime_num {
                    continue;
                }
                let mut tmp = num;
                let mut tmp_count = 0;
                let mut tmp_sum = 1;

                while tmp % prime_num == 0 {
                    tmp /= prime_num;
                    tmp_count += 1;
                    tmp_sum += prime_num.pow(tmp_count);
                }

                count *= (tmp_count + 1);
                sum *= tmp_sum;

                if count > 4 {
                    break;
                }
            }

            if count == 4 {
                result += sum;
            }
        }

        
        result
    }
}