impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_l = 0;
        let mut max_m = 0;
        let mut max_triplet = 0;

        for i in 0 .. nums.len() {
            max_triplet = max_triplet.max(max_m * nums[i] as i64);
            max_m = max_m.max(max_l - nums[i] as i64);
            max_l = max_l.max(nums[i] as i64);
        }
        max_triplet
    }
}