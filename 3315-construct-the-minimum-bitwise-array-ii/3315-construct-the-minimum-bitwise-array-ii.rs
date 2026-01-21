impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::with_capacity(nums.len());

        for num in nums.into_iter() {
            if num == 2 {
                results.push(-1);

                continue;
            }

            let num_bytes = format!("{:b}", num).into_bytes();
            let mut one_trail_count = 0;

            for num_byte in num_bytes.into_iter().rev() {
                if num_byte == b'1' {
                    one_trail_count += 1;
                } else {
                    break;
                }
            }

            if one_trail_count == 0 {
                results.push(-1);
            } else {
                results.push(num - 2_i32.pow(one_trail_count - 1));
            }
        }

        results
    }
}