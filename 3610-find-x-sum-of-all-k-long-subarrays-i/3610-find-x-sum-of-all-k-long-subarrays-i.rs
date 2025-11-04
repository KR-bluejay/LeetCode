use std::collections::{BTreeMap};

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;

        let mut results: Vec<i32> = Vec::with_capacity(nums.len() - k);

        let mut num_map: BTreeMap<i32, i32> = nums[0 .. k - 1]
            .iter()
            .fold(BTreeMap::new(), |mut acc, &item| {
                *acc.entry(item )
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                acc
            });
        
        for (num_id, &num_val) in nums.iter().enumerate().skip(k - 1){
            if k <= num_id {
                num_map.entry(nums[num_id - k] ).and_modify(|v| *v -= 1);
            }

            num_map.entry(num_val)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            let mut temp = 0;

            let mut test: Vec<(i32, i32)> = num_map.clone().into_iter().map(|(k, v)| (k, v)).collect();
            test.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
            let mut temp = 0;
            for item in test.into_iter().take(x as usize) {
                temp += item.0 * item.1;
            }
            results.push(temp);

        }

        results
    }
}