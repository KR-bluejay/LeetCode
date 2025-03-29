impl Solution {
    fn dfs(cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, visited: &mut Vec<bool>, nums: &[i32], len: usize) {
        if cur.len() == len {
            ans.push(cur.to_vec());

            return;
        }

        for i in 0 .. nums.len() {
            cur.push(nums[i]);

            Self::dfs(cur, ans, visited, &nums[i+1..nums.len()], len);

            cur.pop();
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut visited: Vec<bool> = vec![false; nums.len()];
        
        ans.push(Vec::new());

        for i in 1 ..= nums.len() {
            Self::dfs(&mut Vec::with_capacity(nums.len()), &mut ans, &mut visited, &nums, i) ;
            std::mem::replace(&mut visited, vec![false; nums.len()]);
        }

        ans
    }
}