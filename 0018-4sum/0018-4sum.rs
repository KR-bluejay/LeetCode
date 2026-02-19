
use std::collections::HashSet;
use std::cmp::Ordering;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        
        nums.sort_unstable();

        let mut num_id_set: HashSet<(i32, i32, i32, i32)> 
            = HashSet::with_capacity(nums.len());

        for i in 0 .. nums.len() - 3 {
            for j in (i + 1) .. nums.len() - 2 {
                let preset = nums[i] + nums[j];

                let mut left_id = j + 1;
                let mut right_id = nums.len() - 1;

                while left_id < right_id {
                    let sum = preset + nums[left_id] + nums[right_id];

                    match sum.cmp(&target) {
                        Ordering::Less => left_id += 1,
                        Ordering::Greater => right_id -= 1,
                        Ordering::Equal => {
                            num_id_set.insert((
                                nums[i], 
                                nums[j], 
                                nums[left_id], 
                                nums[right_id],
                            ));

                            left_id += 1;
                            right_id -= 1;
                        },
                    }
                }
            }
        }

        num_id_set.into_iter()
            .map(|(a1, a2, a3, a4)| vec![a1, a2, a3, a4])
            .collect()
    }
}