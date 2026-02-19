
use std::collections::HashSet;
use std::cmp::Ordering;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        nums.sort_unstable();

        let target = target as i64;

        let mut num_id_set: Vec<Vec<i32>> 
            = Vec::with_capacity(nums.len());

        for i in 0 .. nums.len() - 3 {
            if 0 < i && nums[i] == nums[i - 1] {
                continue;
            }

            for j in (i + 1) .. nums.len() - 2 {
                if i + 1 < j && nums[j] == nums[j - 1] {
                    continue;
                }

                let preset = nums[i] as i64 + nums[j] as i64;

                let mut left_id = j + 1;
                let mut right_id = nums.len() - 1;

                while left_id < right_id {
                    let sum = preset + nums[left_id] as i64 + nums[right_id] as i64;

                    match sum.cmp(&target) {
                        Ordering::Less => left_id += 1,
                        Ordering::Greater => right_id -= 1,
                        Ordering::Equal => {
                            num_id_set.push(vec![
                                nums[i], 
                                nums[j], 
                                nums[left_id], 
                                nums[right_id],
                            ]);

                            while left_id < right_id
                            && nums[left_id] == nums[left_id + 1] {
                                left_id += 1;
                            }

                            while left_id < right_id
                            && nums[right_id - 1] == nums[right_id] {
                                right_id -= 1;
                            }

                            left_id += 1;
                            right_id -= 1;
                        },
                    }
                }
            }
        }

        num_id_set
    }
}