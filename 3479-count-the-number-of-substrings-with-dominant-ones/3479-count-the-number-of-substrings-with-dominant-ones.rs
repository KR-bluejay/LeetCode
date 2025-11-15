impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let bytes: Vec<u8> = s.into_bytes();
        
        let mut dominant_count: usize = 0;
        
        let mut left_id: usize = 0;
        let mut right_id: usize = 0;

        let mut next_right_id: usize = 0;
        let mut next_one_count = 0;

        let mut zero_count = 0;
        let mut one_count = 0;

        while left_id < bytes.len() {
            right_id = next_right_id.max(left_id);
            zero_count = 0;
            one_count = next_one_count;

            
            while right_id < bytes.len() && bytes[right_id] == b'1' {
                one_count += 1;
                dominant_count += one_count;
                right_id += 1;
            }

            next_right_id = right_id;
            next_one_count = one_count.max(1) - 1;
            
            while right_id < bytes.len() {
                if bytes[right_id] == b'0' {
                    zero_count += 1;
                } else {
                    one_count += 1;
                }

                if zero_count * zero_count <= one_count {
                    dominant_count += 1;
                } else if zero_count * zero_count > one_count + (bytes.len() - right_id - 1) {
                    break;
                }
                right_id += 1;
            }

            left_id += 1;
        }


        dominant_count as i32
    }
}