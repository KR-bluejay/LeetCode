impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut nums: Vec<i32> = vec![0; n];

        if n % 2 == 1 {
            nums[n / 2] = 0;
        }

        for i in 0 .. (n / 2) {
            let rev_id = nums.len() - 1;

            nums[i] = i as i32 + 1;
            nums[rev_id - i]  = nums[i] * - 1;
        }

        nums
    }
}