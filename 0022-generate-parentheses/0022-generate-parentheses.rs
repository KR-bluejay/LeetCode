impl Solution {
    fn backtrack(
        cur: &mut String, 
        remaining_open: i32, 
        remaining_close: i32, 
        ans: &mut Vec<String>
    ) {
        if remaining_open == 0 && remaining_close == 0 {
            ans.push(cur.to_string());

            return;
        }

        if remaining_open > 0 {
            cur.push('(');
            Self::backtrack(cur, remaining_open - 1, remaining_close + 1, ans);
            cur.pop();
        }

        if remaining_close > 0 {
            cur.push(')');
            Self::backtrack(cur, remaining_open, remaining_close - 1, ans);
            cur.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        Self::backtrack(&mut String::with_capacity((n * 2) as usize), n, 0, &mut ans);

        ans
    }
}