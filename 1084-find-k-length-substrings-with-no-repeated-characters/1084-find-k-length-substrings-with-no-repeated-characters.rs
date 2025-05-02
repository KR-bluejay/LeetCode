impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut char_record: Vec<usize> = vec![0; 26];
        let k = k as usize;

        let mut total_no_repeat_count: i32 = 0;

        for i in 0 .. k - 1 {
            let char_num = ((s[i] as u32) - ('a' as u32)) as usize;
            char_record[char_num] += 1;
        }

        for i in k - 1 .. s.len() {
            let char_num = (s[i] as u32 - 'a' as u32) as usize;
            char_record[char_num] += 1;

            if char_record.iter().all(|v| *v <= 1) {
                total_no_repeat_count += 1;
            }

            let first_char_num = (s[i + 1 - k] as u32 - 'a' as u32) as usize;
            char_record[first_char_num] -= 1;
        }
        total_no_repeat_count
    }
}