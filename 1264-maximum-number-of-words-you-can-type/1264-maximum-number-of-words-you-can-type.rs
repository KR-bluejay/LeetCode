impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_chars: Vec<char> = broken_letters.chars().collect();
        let mut split_char = text.split_ascii_whitespace();
        let mut word_count: usize = text.split_ascii_whitespace().count();

        for word in split_char {
            for broken_char in broken_chars.iter() {
                if word.contains(*broken_char) {
                    word_count -= 1;
                    break;
                }
            }
        }
        word_count as i32
    }
}