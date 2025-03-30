use std::collections::{ HashMap };
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s_chars: Vec<char> = s.chars().collect();
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut partition_index = 0;
        let mut output: Vec<i32> = Vec::new();

        for (s_index, s_item) in s_chars.iter().enumerate() {
            *char_map.entry(*s_item).or_insert(0) = s_index
        }

        for (s_index, s_item) in s_chars.iter().enumerate() {
            let last_index = char_map.get(&s_item).unwrap_or(&0);

            partition_index = partition_index.max(*last_index);

            if partition_index == s_index {
                output.push(partition_index as i32);
            }
        }

        for item in output.iter_mut() {
            *item += 1;
        }

        for i in (1 .. output.len()).rev() {
            output[i] -= output[i - 1];
        }

        


        output
    }
}