impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x_char: Vec<char> = x.to_string().chars().collect();

        for i in 0 .. x_char.len() / 2 {
            if x_char[i] != x_char[x_char.len() - i - 1] {
                return false;
            }
        }

        true
    }
}