impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut cache = vec![false; 52];
        let mut result = 0;

        for word_byte in word.into_bytes() {
            if word_byte <= b'Z' {
                let id = (word_byte - b'A' + 26) as usize;
                let rev_id = (word_byte - b'A') as usize;

                if cache[rev_id] && !cache[id] {
                    result += 1;
                }
                cache[id] = true;
            } else {
                let id = (word_byte - b'a') as usize;
                let rev_id = (word_byte - b'a' + 26) as usize;

                if cache[id] && cache[rev_id] {
                    result -= 1;
                }
                cache[id] = !cache[rev_id];
            }
        }

        result
    }
}