impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let s_bytes = s.into_bytes();

        let mut byte_seen: u32 = 0;
        let mut byte_pos: [usize; 26] = [0; 26];
        let mut byte_stack: Vec<u8> = Vec::with_capacity(s_bytes.len().min(26));
        
        for (id, &s_byte) in s_bytes.iter().enumerate() {
            byte_pos[(s_byte - b'a') as usize] = id;
        }

        for (id, s_byte) in s_bytes.into_iter().enumerate() {
            let s_id = s_byte - b'a';

            if byte_seen & (1 << s_id) != 0 {
                continue;
            }

            while let Some(&last_char) = byte_stack.last() 
            && last_char > s_byte 
            && byte_pos[(last_char - b'a') as usize] > id {
                byte_stack.pop();
                byte_seen &= !(1 << (last_char - b'a'));
            }
            byte_stack.push(s_byte);
            byte_seen |= (1 << s_id);
        }

        unsafe{String::from_utf8_unchecked(byte_stack)}
        
    }
}