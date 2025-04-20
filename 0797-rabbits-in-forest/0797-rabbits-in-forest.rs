use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut color_map: HashMap<i32, i32> = HashMap::new();
        let mut rabbit_count: i32 = 0;

        for &answer_item in answers.iter() {
            let mut res = *color_map.entry(answer_item).and_modify(|v| *v += 1).or_insert(0);

            if res == answer_item {
                rabbit_count += (answer_item + 1);
                color_map.remove(&answer_item);
            }
        }

        for (color_key, color_count) in color_map.iter() {
            rabbit_count += color_key + 1;
        }

        rabbit_count
    }
}