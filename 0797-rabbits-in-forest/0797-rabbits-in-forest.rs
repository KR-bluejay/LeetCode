use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut color_map: HashMap<i32, i32> = HashMap::new();
        let mut rabbit_count: i32 = 0;

        for &answer_item in answers.iter() {
            let mut group_size = color_map.entry(answer_item)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            if *group_size == (answer_item + 1) {
                *group_size = 0;
                rabbit_count += (answer_item + 1);
            }
        }


        for (color_key, &color_count) in color_map.iter() {
            if color_count != 0 {
                rabbit_count += color_key + 1;
            }
        }

        rabbit_count
    }
}