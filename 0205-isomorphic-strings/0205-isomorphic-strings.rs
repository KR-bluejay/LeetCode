use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut source_map: HashMap<char, char> = HashMap::new();
        let mut target_map: HashMap<char, char> = HashMap::new();


        for (source_char, target_char) in s.chars().zip(t.chars()) {
            let mapped_target = source_map.entry(source_char).or_insert(target_char);
            let mapped_source = target_map.entry(target_char).or_insert(source_char);


            if *mapped_target != target_char || *mapped_source != source_char {
                return false;
            }
        }

        return true;
    }
}