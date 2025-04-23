impl Solution {
    fn try_interleave(
        s1: &Vec<char>, 
        s2: &Vec<char>, 
        s3: &Vec<char>,
        s1_id: usize, 
        s2_id: usize, 
        s3_id: usize,
        dp: &mut Vec<Vec<bool>>
    ) -> bool {
        if s3_id == s3.len() {
            return s1_id == s1.len() && s2_id == s2.len();
        }

        if dp[s1_id][s2_id] == false {
            return false;
        }

        if s1_id < s1.len() && s1[s1_id] == s3[s3_id] 
        && Self::try_interleave(
            s1, 
            s2, 
            s3, 
            s1_id + 1, 
            s2_id, 
            s3_id + 1, 
            dp
        ) {
            return true;
        }

        if s2_id < s2.len() && s2[s2_id] == s3[s3_id] 
        && Self::try_interleave(
                s1, 
                s2, 
                s3, 
                s1_id, 
                s2_id + 1, 
                s3_id + 1, 
                dp
            ) {
            return true;
        }

        dp[s1_id][s2_id] = false;
        false
    }
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        let mut dp: Vec<Vec<bool>> = vec![vec![true; s2.len() + 1]; s1.len() + 1];


        Self::try_interleave(&s1, &s2, &s3, 0, 0, 0, &mut dp);

        dp[0][0]
    }
}