impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.into_bytes();

        let mut left = 0;
        let mut right = 0;
        let mut total = 0;

        let mut pos = vec![0; 3];

        while right < s.len() {
            pos[(s[right] - b'a') as usize] += 1;

            while pos.iter().all(|&p| p > 0) {
                total += s.len() - right;


                pos[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }

            right += 1;
        }

        total as i32
    }
}