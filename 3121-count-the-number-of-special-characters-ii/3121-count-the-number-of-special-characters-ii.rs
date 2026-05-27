impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let word = word.into_bytes();
        let mut result = 0;

        for byte_id in b'a' ..= b'z' {
            if let Some(lower_id) = word.iter().rposition(|&w| w == byte_id) 
            && let Some(upper_id) = word.iter().position(|&w| w == byte_id - 32) 
            && lower_id < upper_id {
                result += 1;
            }
        }

        result
    }
}