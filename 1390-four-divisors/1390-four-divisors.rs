impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums.into_iter() {
            let mut num_sum = 1 + num;
            let mut num_count = 2;


            for pair_num in 2 ..= num.isqrt() {
                if num % pair_num != 0 {
                    continue;
                }

                num_count += 1;
                num_sum += num / pair_num;

                if num / pair_num != pair_num {
                    num_count += 1;
                    num_sum += pair_num;
                }

                if num_count > 4 {
                    break;
                }
            }

            if num_count == 4 {
                result += num_sum;
            }
        }

        result
    }
}