impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();

        let mut zero_count = 0;
        let mut one_count = 0;
        let mut total_count = 0;

        for i in 0 .. s.len() {
            zero_count = 0;
            one_count = 0;
            
            for j in i .. s.len() {
                if s[j] == '1' {
                    one_count += 1;
                } else {
                    zero_count += 1;
                }

                if zero_count <= k || one_count <= k {
                    total_count += 1;
                } else if zero_count > k && one_count > k{
                    break;
                }
            }
        }
        total_count

    }
}