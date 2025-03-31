use std::cmp;

impl Solution {
    fn dp(
        txt1_id: usize, 
        txt2_id: usize, 
        txt1_chars: &Vec<char>, 
        txt2_chars: &Vec<char>, 
        memo: &mut Vec<Vec<i32>>
    ) -> i32 {
        if txt1_id == txt1_chars.len() || txt2_id == txt2_chars.len() {
            return 0;
        }

        if memo[txt1_id][txt2_id] != -1 {
            return memo[txt1_id][txt2_id];
        }

        memo[txt1_id][txt2_id] = if txt1_chars[txt1_id] == txt2_chars[txt2_id] {
            1 + Self::dp(txt1_id + 1, txt2_id + 1, txt1_chars, txt2_chars, memo)
        } else {
            cmp::max(
                Self::dp(txt1_id + 1, txt2_id, txt1_chars, txt2_chars, memo),
                Self::dp(txt1_id, txt2_id + 1, txt1_chars, txt2_chars, memo)
            )
        };

        memo[txt1_id][txt2_id]
    }
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; text2.len()]; text1.len()];
        let txt1_chars: Vec<char> = text1.chars().collect();
        let txt2_chars: Vec<char> = text2.chars().collect();

        Self::dp(0, 0, &txt1_chars, &txt2_chars, &mut memo)
    }
}