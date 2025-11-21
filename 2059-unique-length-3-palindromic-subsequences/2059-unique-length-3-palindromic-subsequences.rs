impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut bytes: Vec<u8> = s.into_bytes();
        let mut first_ids = vec![usize::MAX; 26];
        let mut last_ids = vec![0; 26];
        let mut result = 0;

        for id in 0 .. bytes.len() {
            let char_id = (bytes[id] - 97) as usize;

            if first_ids[char_id] == usize::MAX {
                first_ids[char_id] = id;
            }
            last_ids[char_id] = id;
        }

        for id in 0 .. 26 {
            let mut count = 0;
            let mut visit: u32 = 0;


            for second_id in (first_ids[id] + 1) .. last_ids[id] {
                let char_id = (bytes[second_id] - 97) as usize;

                if count == 26 {
                    break;
                }
                
                if visit & (1 << char_id) != 0 {
                    continue;
                }

                visit |= 1 << char_id;
                count += 1;
            }

            result += count;
        }

        result
    }
}