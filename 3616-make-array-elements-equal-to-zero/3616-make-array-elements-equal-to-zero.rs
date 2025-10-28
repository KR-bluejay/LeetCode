impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = 0;
        let mut sum: i32 = nums.iter().sum();
        let mut count = 0;

        for (id, &num_val) in nums.iter().enumerate() {
            sum -= num_val;
            prefix_sum += num_val;

            if num_val == 0 && prefix_sum == sum {
                count += 2;
            } else if num_val == 0 && (prefix_sum - sum).abs() == 1 {
                println!("{id} {prefix_sum} {sum}");
                count += 1;
            }
        }


        count
    }
}