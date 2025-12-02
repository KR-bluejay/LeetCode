impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s_bytes: Vec<u8> = s.into_bytes();
        let mut result = 0;

        for id in 0 .. s_bytes.len() / 2 {
            if s_bytes[2 * id] != s_bytes[2 * id + 1] {
                result += 1;
            }
        }

        result
    }
}