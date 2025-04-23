impl Solution {
    fn find_palindrome(
        s: &Vec<char>, 
        initial_left_id: usize, 
        initial_right_id: usize
    ) -> (usize, usize) {
        let mut left_id = initial_left_id as i32;
        let mut right_id = initial_right_id;

        while 0 <= left_id 
            && right_id < s.len() 
            && s[left_id as usize] == s[right_id] 
        {
            // Underflow
            left_id -= 1;
            right_id += 1;
        }

        (((left_id + 1) as usize), (right_id - 1))
    }
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let s: Vec<char> = s.chars().collect();
       
        let mut long_left_id = 0;
        let mut long_right_id = 0;

        for i in 0 .. s.len() {
            let (left_id, right_id) = Self::find_palindrome(&s, i, i);


            if left_id <= right_id 
                && (right_id - left_id) > (long_right_id - long_left_id) {
                long_left_id = left_id;
                long_right_id = right_id;
            }

            if i + 1 == s.len() {
                continue;
            }

            let (left_id, right_id) = Self::find_palindrome(&s, i, i + 1);
            if left_id <= right_id && 
                (right_id - left_id) > (long_right_id - long_left_id) {
                long_left_id = left_id;
                long_right_id = right_id;
            }
        }


        s[long_left_id ..= long_right_id].iter().collect()
    }
}