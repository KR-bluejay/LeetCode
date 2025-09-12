impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let bowl_count: usize = s.chars().filter(|v| {
            match (*v).to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false
            }
        }).count();

        bowl_count != 0
    }
}