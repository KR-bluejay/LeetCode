use std::collections::{ BTreeMap };

impl Solution {
    pub fn insert(
        intervals: Vec<Vec<i32>>, 
        new_interval: Vec<i32>
    ) -> Vec<Vec<i32>> {
        let mut interval_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut prefix_sum = 0;
        let mut prev_num = i32::MAX;
        let mut output: Vec<Vec<i32>> = Vec::new();

        for interval_item in intervals.iter().chain(std::iter::once(&new_interval)) {
            let start_num = interval_item[0];
            let end_num = interval_item[1];

            *interval_map.entry(start_num).or_insert(0) += 1;
            *interval_map.entry(end_num).or_insert(0) -= 1;
        }

        for (k, v) in interval_map.iter() {
            prev_num = prev_num.min(*k);
            prefix_sum += *v;

            if prefix_sum == 0 {
                output.push(vec![prev_num, *k]);
                prev_num = i32::MAX;
            }
        }
        output
    }
}