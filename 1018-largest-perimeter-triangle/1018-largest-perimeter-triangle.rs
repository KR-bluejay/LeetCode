impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(&a));

        for i in 0 .. nums.len() - 2 {
            let a = nums[i];
            let b = nums[i + 1];
            let c = nums[i + 2];

            if b + c > a {
                return a + b + c;
            }
        }
        0
    }
}