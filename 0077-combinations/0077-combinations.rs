impl Solution {
    fn backtracking(cur: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, cur_num: i32, max_num: i32, num_len: i32) {
        if cur.len() == num_len as usize {
            ans.push(cur.to_vec());

            return;
        }

        for i in cur_num ..= max_num {
            cur.push(i);

            Self::backtracking(cur, ans, i + 1, max_num, num_len);

            cur.pop();
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();


        Self::backtracking(&mut Vec::new(), &mut ans, 1, n, k);
        ans
    }
}