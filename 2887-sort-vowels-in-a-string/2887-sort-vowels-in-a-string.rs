
impl Solution {
    pub fn sort_vowels(mut s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut conso_ids: Vec<usize> = Vec::with_capacity(s.len());
        let mut conso_values: Vec<u8> = Vec::with_capacity(s.len());

        for (s_id, s_char) in s.iter().enumerate() {
            match s_char.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    conso_ids.push(s_id);
                    conso_values.push(*s_char as u8);
                },
                _ => {
                    continue;
                },
            }
        }

        conso_values.sort();

        for (k, v) in conso_ids.iter().enumerate() {
            s[*v] = conso_values[k] as char;
        }

        s.iter().collect()
    }
}