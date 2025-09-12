impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.chars().filter(|v| {
            match *v {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false
            }
        }).count() != 0
    }
}