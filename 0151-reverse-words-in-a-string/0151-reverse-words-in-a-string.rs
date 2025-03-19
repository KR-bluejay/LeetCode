impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        let mut rev_str = String::new();
        let mut rev_iter = s.split_whitespace().rev().peekable();

        while let Some(next_str) = rev_iter.next() {
            rev_str += next_str;
            
            if rev_iter.peek().is_some() {
                rev_str += " ";
            }
        }

        rev_str
    }
}