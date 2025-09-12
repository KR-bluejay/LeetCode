impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.chars().filter(|v| {
            match (*v).to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false
            }
        }).count() != 0
    }
}