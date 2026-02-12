impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let bytes: Vec<u8> = s.into_bytes().into_iter().map(|v| v - b'a').collect();
        let mut byte_count = vec![0; 26];
        let mut result = 0;


        for left_id in 0 .. bytes.len() {
            for right_id in left_id .. bytes.len() {
                byte_count[bytes[right_id] as usize] += 1;
                let ref_count = byte_count[bytes[right_id] as usize];

                if byte_count.iter().all(|&v| v == 0 || ref_count == v) {
                    result = result.max(right_id - left_id + 1);
                }
            }
            
            byte_count = vec![0; 26];
        }

        result as i32
    }
}