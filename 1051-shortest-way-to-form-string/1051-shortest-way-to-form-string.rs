use std::collections::{ HashSet };

impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let source_list: Vec<char> = source.chars().collect();
        let target_list: Vec<char> = target.chars().collect();

        let source_set: HashSet<char> = source_list.iter().cloned().collect();

        let mut subsequent_count = 0;
        let mut source_index = 0;


        if !target_list.iter().all(|c| source_set.contains(c)) {
            return -1;
        }


        for target_item in target_list.iter() {
            if source_index == 0 {
                subsequent_count += 1;
            }

            while *target_item != source_list[source_index] {
                source_index = (source_index + 1) % source_list.len();

                if source_index == 0 {
                    subsequent_count += 1;
                }
            }
            source_index = (source_index + 1) % source_list.len();
        }
        subsequent_count
    }
}