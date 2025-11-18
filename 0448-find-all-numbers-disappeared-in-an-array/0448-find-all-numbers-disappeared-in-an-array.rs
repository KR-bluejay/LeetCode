impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let mut disappeared_nums: Vec<i32> = Vec::with_capacity(nums.len());
        let mut prev_num = 0;

        for &num in nums.iter() {
            if num.min(prev_num) + 1 != num.max(prev_num) {
                for tmp in num.min(prev_num) + 1 .. num.max(prev_num) {
                    disappeared_nums.push(tmp);
                }
            }
            prev_num = num;
        }

        for tmp in *nums.last().unwrap() + 1 ..= nums.len() as i32 {
            disappeared_nums.push(tmp);
        }

        disappeared_nums
    }
}