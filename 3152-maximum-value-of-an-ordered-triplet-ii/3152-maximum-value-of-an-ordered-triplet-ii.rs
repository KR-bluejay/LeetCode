impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max_l: i64 = 0;
        let mut max_m: i64 = 0;
        let mut max_triplet: i64 = 0;

        for i in 0 .. nums.len() {
            let cur_num = nums[i] as i64;

            max_triplet = max_triplet.max(max_m * cur_num);
            max_m = max_m.max(max_l - cur_num);
            max_l = max_l.max(cur_num);
        }
        max_triplet
    }
}