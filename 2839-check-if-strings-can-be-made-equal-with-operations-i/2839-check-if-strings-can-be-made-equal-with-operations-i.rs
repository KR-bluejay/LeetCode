impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1_bytes = s1.into_bytes();
        let mut s2_bytes = s2.into_bytes();

        for id in 0 .. s1_bytes.len() {
            if s1_bytes[id] == s2_bytes[id] {
                continue;
            }
            
            if id + 2 >= s1_bytes.len() || s1_bytes[id] != s2_bytes[id + 2] {
                return false;
            }

            s2_bytes[id + 2] = s2_bytes[id];
        }

        true
    }
}