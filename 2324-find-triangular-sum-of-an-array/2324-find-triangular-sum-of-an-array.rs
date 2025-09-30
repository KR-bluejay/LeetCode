impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let mut triangle_len = nums.len();
        
        while triangle_len > 1 {
            for i in 0 .. triangle_len - 1 {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            triangle_len -= 1;
        }

        nums[0]
    }
}