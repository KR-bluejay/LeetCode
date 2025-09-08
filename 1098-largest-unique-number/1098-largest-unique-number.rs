use std::collections::BTreeMap;

impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        let mut num_map: BTreeMap<i32, usize> = BTreeMap::new();

        for num in nums.iter() {
            num_map.entry(*num).and_modify(|v| *v += 1).or_insert(1);
        }

        num_map.into_iter().filter(|(k, v)| *v == 1).map(|(k, _)| k).last().unwrap_or(-1)
    }
}