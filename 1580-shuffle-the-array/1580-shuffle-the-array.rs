impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut shuffled_nums: Vec<i32> = Vec::with_capacity(nums.len());

        for id in 0 .. nums.len() - n {
            shuffled_nums.push(nums[id]);

            if id + n < nums.len() {
                shuffled_nums.push(nums[id + n]);
            }
        }


        shuffled_nums
    }
}