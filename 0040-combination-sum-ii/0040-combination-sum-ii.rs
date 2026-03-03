use std::collections::HashSet;
use std::cmp::Ordering;

impl Solution {
    fn backtrack(
        result_set: &mut HashSet<Vec<i32>>,
        nums: &Vec<i32>,
        num_id: usize,
        combination: &mut Vec<i32>,
        remaining: i32,
    ) {
        for next_id in num_id .. nums.len() {
            match nums[next_id].cmp(&remaining) {
                Ordering::Less => {
                    if num_id != next_id 
                    && nums[num_id] == nums[next_id] {
                        continue;
                    }
                    
                    combination.push(nums[next_id]);

                    Self::backtrack(
                        result_set, 
                        nums, 
                        next_id + 1, 
                        combination, 
                        remaining - nums[next_id]
                    );

                    combination.pop();
                },
                Ordering::Equal => {
                    combination.push(nums[next_id]);

                    result_set.insert(combination.to_vec());

                    combination.pop();
                },
                Ordering::Greater => {}
            }
        }
    }
    pub fn combination_sum2(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();


        let mut result_set: HashSet<Vec<i32>> = HashSet::with_capacity(nums.len());
        let mut combination: Vec<i32> = Vec::with_capacity(nums.len());

        Self::backtrack(
            &mut result_set,
            &nums,
            0,
            &mut combination,
            target
        );

        result_set.into_iter().collect()
    }
}