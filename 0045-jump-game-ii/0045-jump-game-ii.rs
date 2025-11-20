impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur_end = 0;
        let mut cur_farthest = 0;

        let mut jump = 0;

        for (id, &num) in nums.iter().enumerate().take(nums.len() - 1) {
            let num = num as usize;

            cur_farthest = cur_farthest.max(id + num);

            if id == cur_end {
                jump += 1;
                cur_end = cur_farthest;

                if nums.len() - 1 <= cur_end {
                    break;
                }
            }
        }

        jump
    }
}