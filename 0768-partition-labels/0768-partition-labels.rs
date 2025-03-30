use std::collections::{ HashMap };
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s_chars: Vec<char> = s.chars().collect();
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut partition_start = 0;
        let mut partition_end = 0;
        let mut partition_sizes: Vec<i32> = Vec::new();

        for (s_index, s_item) in s_chars.iter().enumerate() {
            *char_map.entry(*s_item).or_insert(0) = s_index
        }

        for (s_index, s_item) in s_chars.iter().enumerate() {
            let last_index = char_map.get(&s_item).unwrap_or(&0);

            partition_end = partition_end.max(*last_index);

            if partition_end == s_index {
                let partition_size = partition_end - partition_start + 1;

                partition_sizes.push(partition_size as i32);
                
                partition_start = s_index + 1;
            }
        }



        partition_sizes
    }
}