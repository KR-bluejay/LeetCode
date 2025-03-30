impl Solution {
    pub fn shortest_way(source: String, target: String) -> i32 {
        let source_list: Vec<char> = source.chars().collect();
        let target_list: Vec<char> = target.chars().collect();


        let mut form_count = 0;
        let mut target_index = 0;

        while target_index < target_list.len() && form_count <= target_list.len() {
            form_count += 1;

            for source_item in source_list.iter() {
                if target_index >= target_list.len() {
                    break;
                }

                if *source_item == target_list[target_index] {
                    target_index += 1;
                }
            }
        }

        if form_count >= target_list.len() {
            return -1;
        }

        form_count as i32
    }
}