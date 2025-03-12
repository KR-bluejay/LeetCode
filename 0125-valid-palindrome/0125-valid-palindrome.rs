impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        s.retain(|item| item.is_alphanumeric());
        s = s.to_lowercase();

        for i in 0 .. s.len() / 2 {
            let first_item = s.chars().nth(i);
            let last_item = s.chars().nth(s.len() - 1 - i);
            println!("{first_item:?} {last_item:?}");

            if (first_item != last_item) {
                return false
            }
        }

        return true;
    }
}