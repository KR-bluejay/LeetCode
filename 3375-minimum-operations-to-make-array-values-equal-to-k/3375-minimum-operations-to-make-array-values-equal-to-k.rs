impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let mut op_count = 0;
        let mut last_num = nums[nums.len() - 1]; 


        if !nums.iter().all(|v| *v >= k) {
            return -1;
        }

        for i in (0 .. nums.len()).rev() {
            if last_num != nums[i] {
                last_num = nums[i];
                op_count += 1;
            }
        }

        if nums[0] != k {
            op_count += 1;
        }

        op_count
    }
}