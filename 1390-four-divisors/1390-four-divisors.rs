impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut divisor = Vec::with_capacity(nums.len());

        for num in nums.into_iter() {
            let mut num_sum = 1 + num;

            divisor.push(1);
            divisor.push(num);

            for pair_num in 2 ..= num.isqrt() {
                if num % pair_num != 0 {
                    continue;
                }

                num_sum += num / pair_num;
                divisor.push(num / pair_num);

                if num / pair_num != pair_num {
                    num_sum += pair_num;
                    divisor.push(pair_num);
                }

                if divisor.len() > 4 {
                    break;
                }
            }

            if divisor.len() == 4 {
                result += num_sum;
            }
            divisor.clear();
        }

        result
    }
}