use std::collections::HashMap;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut vowel_map: HashMap<char, usize> = HashMap::with_capacity(5);
        let mut consonant_map: HashMap<char, usize> = HashMap::with_capacity(21);
        
        let mut vowel_max_count = 0;
        let mut con_max_count = 0;

        for s_char in s.chars() {
            match s_char  {
                'a' | 'e' | 'i' |'o' | 'u' => {
                    vowel_map.entry(s_char).and_modify(|v| *v += 1).or_insert(1);
                },
                _ => {
                    consonant_map.entry(s_char).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }

        for (k, v) in vowel_map.iter() {
            vowel_max_count = vowel_max_count.max(*v);
        }

        for (k, v) in consonant_map.iter() {
            con_max_count = con_max_count.max(*v);
        }

        (vowel_max_count + con_max_count) as i32
    }
}