impl Solution {
    fn dfs(cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        if cur.len() == nums.len() {
            ans.push(cur.to_vec());

            return;
        }

        for &num_item in nums.iter() {
            if cur.contains(&num_item) {
                continue;
            }
            cur.push(num_item);
            Self::dfs(cur, ans, nums);
            cur.pop();
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        Self::dfs(&mut Vec::with_capacity(nums.len()), &mut ans, &nums);

        ans
    }
}