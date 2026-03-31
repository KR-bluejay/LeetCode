impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let str1 = str1.into_bytes();
        let str2 = str2.into_bytes();

        let mut word = vec![b'a'; str1.len() + str2.len() - 1];
        let mut word_fixed = vec![false; str1.len() + str2.len() - 1];

        for (id, _) in str1.iter().enumerate()
            .filter(|(id, s)| **s == b'T') {
            for word_id in id .. id + str2.len() {
                if word_fixed[word_id] 
                && word[word_id] != str2[word_id - id] {
                    return String::new();
                }

                word[word_id] = str2[word_id - id];
                word_fixed[word_id] = true;
            }
        }

        for (id, _) in str1.iter().enumerate()
            .filter(|(id, s)| **s == b'F') {
            let mut is_mismatch = false;
            let mut replace_id = usize::MAX;

            for word_id in (id .. id + str2.len()).rev() {
                is_mismatch |= word[word_id] != str2[word_id - id];

                if replace_id == usize::MAX 
                && !word_fixed[word_id] {
                    replace_id = word_id;
                }
            }

            if is_mismatch {
                continue;
            } else if replace_id < usize::MAX {
                word[replace_id] = b'b';
            } else {
                return String::new();
            }
        }

        unsafe { String::from_utf8_unchecked(word) }
    }
}