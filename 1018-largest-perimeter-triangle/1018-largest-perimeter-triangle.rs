impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(&a));

        for i in 0 .. nums.len() - 2 {
            let a = nums[i];
            for j in i + 1 .. nums.len() - 1 {
                let b = nums[j];

                for k in j + 1 .. nums.len() {
                    let c = nums[k];

                    if b + c > a {
                        return a + b + c;
                    } else {
                        break;
                    }
                }
            }
        }
        0
    }
}