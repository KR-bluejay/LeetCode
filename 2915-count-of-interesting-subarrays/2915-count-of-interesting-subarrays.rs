use std::collections::{BTreeMap};
impl Solution {
    pub fn count_interesting_subarrays(
        nums: Vec<i32>, 
        modulo: i32, 
        k: i32
    ) -> i64 {
        let mut prefix_sum = 0;
        let mut count_map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut total_sum = 0;

        count_map.insert(0, 1);

        for (num_id, &num_val) in nums.iter().enumerate() {
            prefix_sum += if num_val % modulo == k {
                1
            } else {
                0
            };
            
            let target = (prefix_sum - k + modulo) % modulo;


            if let Some(v) = count_map.get(&target) {
                total_sum += *v as i64
            }
            count_map.entry(prefix_sum % modulo)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        total_sum
    }
}