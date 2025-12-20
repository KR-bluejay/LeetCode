impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let bytes: Vec<Vec<u8>> = strs.into_iter().map(|v| v.into_bytes()).collect();
        let mut result = 0;

        for id in 0 .. bytes[0].len() {
            let mut a_id = 0;
            for b_id in 0 .. bytes.len() {
                if a_id <= bytes[b_id][id] {
                    a_id =  bytes[b_id][id];
                } else {
                    result += 1;
                    break;
                }
            }
        }

        result
    }
}