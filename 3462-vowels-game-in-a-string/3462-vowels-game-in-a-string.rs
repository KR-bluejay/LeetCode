impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.bytes().any(|v| matches!(v | 32, b'a' | b'e' | b'i' | b'o' | b'u'))
    }
}