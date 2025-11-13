impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut bytes: Vec<u8> = s.into_bytes().into_iter().map(|v| v - b'0').collect();
        let mut op_count = 0;

        let mut one_count = bytes[0] as i32;

        let mut is_add = false;

        for id in 1 .. bytes.len() {
            if bytes[id] == 1 {
                one_count += 1;
            } else if bytes[id - 1] == 1 {
                op_count += one_count;
            }
        }


        op_count
    }
}