impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut op_count = 0;

        for i in 0 .. nums.len() - 2 {
            if nums[i] == 0 {
                for j in i ..= i + 2 {
                    nums[j] = if nums[j] == 0 {
                        1
                    } else {
                        0
                    };
                }
                op_count += 1;
            }
        }

        for i in nums.len() - 3 .. nums.len() {
            if nums[i] != 1 {
                return -1;
            }
        }
        op_count
    }
}