use std::collections::HashMap;
use itertools::Itertools;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut alpha_count_list = vec![0; 26];
        let mut word_map: HashMap<String, Vec<String>> = HashMap::new();


        for str_item in strs {
            let char_list = str_item.chars();

            for char_item in char_list {
                alpha_count_list[char_item as usize - 'a' as usize] += 1;
            }

            let key: String = alpha_count_list.iter().map(|x| x.to_string()).join("_");
            word_map.entry(key).or_insert(Vec::new()).push(str_item);

            alpha_count_list = vec![0; 26];
        }

        let mut output_vec: Vec<Vec<String>> = Vec::new();


        for (_, v) in word_map.iter() {
            output_vec.push(v.clone());
        }

        return output_vec;
    }
}