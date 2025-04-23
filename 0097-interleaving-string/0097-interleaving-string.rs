impl Solution {
    fn try_interleave(
        s1: &Vec<char>, 
        s2: &Vec<char>, 
        s3: &Vec<char>,
        s1_id: usize, 
        s2_id: usize, 
        s3_id: usize,
        dp: &mut Vec<Vec<Option<bool>>>
    ) -> Option<bool> {
        println!("{s1_id} {s2_id} {s3_id}");
        if s3_id == s3.len() {
            println!("A");
            return Some(s1_id == s1.len() && s2_id == s2.len());
        }

        if let Some(is_interleave) = dp[s1_id][s2_id] {
            return Some(is_interleave);
        }

        let s1_result = s1_id < s1.len() && s1[s1_id] == s3[s3_id] &&
            Self::try_interleave(
                s1, 
                s2, 
                s3, 
                s1_id + 1, 
                s2_id, 
                s3_id + 1, 
                dp
            ).unwrap();

        let s2_result = s2_id < s2.len() && s2[s2_id] == s3[s3_id] && 
            Self::try_interleave(
                s1, 
                s2, 
                s3, 
                s1_id, 
                s2_id + 1, 
                s3_id + 1, 
                dp
            ).unwrap();

        dp[s1_id][s2_id] = Some(s1_result || s2_result);
        dp[s1_id][s2_id]
    }
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();

        let mut dp: Vec<Vec<Option<bool>>> = vec![vec![None; s2.len() + 1]; s1.len() + 1];


        Self::try_interleave(&s1, &s2, &s3, 0, 0, 0, &mut dp).unwrap()
    }
}