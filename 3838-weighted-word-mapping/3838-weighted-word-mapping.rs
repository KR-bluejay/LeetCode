impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut result = vec![0; words.len()];

        for (word_id, word) in words.into_iter().enumerate() {
            for word_byte in word.into_bytes() {
                let id = (word_byte - b'a') as usize;

                result[word_id] += weights[id] as u8;
                result[word_id] %= 26;
            }

            result[word_id] = b'a'+ 25 - result[word_id];
        }

        unsafe { String::from_utf8_unchecked(result) }
    }
}