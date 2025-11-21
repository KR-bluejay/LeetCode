impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let bytes: Vec<u8> = s.into_bytes();
        let mut visit: Vec<bool> = vec![false; 26];

        let mut last_ids: Vec<usize> = vec![0; 26];

        let mut result = 0;

        for id in (0 .. bytes.len()).rev() {
            let char_id = (bytes[id] - 97) as usize;
            
            if last_ids[char_id] == 0 {
                last_ids[char_id] = id;
            }
        }

        for id in (0 .. bytes.len()) {
            let char_id = (bytes[id] - 97) as usize;
            let mut tmp_visit: Vec<bool> = vec![false; 26];
            let mut count = 0;

            if visit[char_id] {
                continue;
            }
            visit[char_id] = true;

            for second_id in ((id + 1) .. last_ids[char_id]) {
                let second_char_id = (bytes[second_id] - 97) as usize;

                if count == 26 {
                    break;
                }

                if tmp_visit[second_char_id] {
                    continue;
                }

                count += 1;
                tmp_visit[second_char_id] = true;
            }


            result += count;
        }


        result
    }
}