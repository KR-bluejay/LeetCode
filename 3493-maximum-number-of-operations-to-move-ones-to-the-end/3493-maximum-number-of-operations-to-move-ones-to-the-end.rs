impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut bytes: Vec<u8> = s.into_bytes().into_iter().map(|v| v - b'0').collect();
        let mut op_count = 0;

        let mut left_id = 0;

        while left_id < bytes.len() {
            while left_id < bytes.len() && bytes[left_id] == 0 {
                left_id += 1;
            }
            
            let mut right_id = left_id;

            while right_id + 1 < bytes.len() && bytes[right_id + 1] == 0 {
                right_id += 1;
            }

            if left_id != right_id {
                op_count += 1;
                bytes.swap(left_id, right_id);
            }

            left_id = right_id + 1;
        }
        
        left_id = 0;
        let mut cur_count = 0;

        while left_id < bytes.len() {
            while left_id < bytes.len() && bytes[left_id] == 0 {
                left_id += 1;
            }
            
            let mut right_id = left_id;

            while right_id < bytes.len() && bytes[right_id] == 1 {
                right_id += 1;
            }

            if left_id != right_id && right_id < bytes.len() {
                let temp = right_id - left_id;
                cur_count += temp;
                op_count += cur_count;
            }

            left_id = right_id + 1;
        }


        op_count as i32
    }
}