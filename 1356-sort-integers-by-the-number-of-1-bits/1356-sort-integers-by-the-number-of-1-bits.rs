use std::collections::BTreeMap;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        let mut num_map: BTreeMap<u32, Vec<i32>> = BTreeMap::new();

        arr.sort_unstable();

        for num in arr.into_iter() {
            num_map.entry(num.count_ones()).or_insert(Vec::new()).push(num);
        }

        num_map.into_values().flatten().collect()
    }
}