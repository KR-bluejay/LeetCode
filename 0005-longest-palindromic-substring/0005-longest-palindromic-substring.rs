impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s: Vec<char> = s.chars().collect();

        let mut cache: Vec<Vec<Option<bool>>> = vec![vec![None; s.len()]; s.len()];

        let mut long_str_len: usize = 0;
        let mut long_str_start: usize = 0;
        let mut long_str_end: usize = 0;

        for i in 1 .. s.len()  {
            let mid_id = i;
            let mut left_id = i - 1;
            let mut right_id = i + 1;

            while left_id >= 0 && right_id < s.len() && left_id < s.len()  {
                if cache[left_id][right_id].is_some() &&  !cache[left_id][right_id].unwrap(){
                    break;
                }
    
                if s[left_id] != s[right_id] {
                    cache[left_id][right_id] = Some(false);
                    break;
                }
                if long_str_len < right_id - left_id + 1 {
                    long_str_start = left_id;
                    long_str_end = right_id;
                    long_str_len = right_id - left_id + 1 
                }
                cache[left_id][right_id] = Some(true);
                left_id -= 1;
                right_id += 1;
            }

            left_id = mid_id;
            right_id = mid_id - 1;

            while left_id >= 0 && right_id < s.len() && left_id < s.len()  {
                if cache[left_id][right_id].is_some() && !cache[left_id][right_id].unwrap() {
                    break;
                }
    
                if s[left_id] != s[right_id] {
                    cache[left_id][right_id] = Some(false);
                    break;
                }

                if long_str_len <= right_id - left_id + 1 {
                    long_str_start = left_id;
                    long_str_end = right_id;
                    long_str_len = right_id - left_id + 1;
                }
                cache[left_id][right_id] = Some(true);
                left_id -= 1;
                right_id += 1;
            }
        }
        

        s[long_str_start ..= long_str_end].iter().collect()
    }
}