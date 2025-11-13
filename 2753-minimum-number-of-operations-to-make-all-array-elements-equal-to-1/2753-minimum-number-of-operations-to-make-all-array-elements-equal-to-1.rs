use std::collections::{HashMap};

impl Solution {
    fn gcd(mut lhs: i32, mut rhs: i32) -> i32 {
        while rhs != 0 {
            let temp = rhs;
            
            rhs = lhs % rhs;
            lhs = temp;
        }

        lhs
    }
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut num_map: HashMap<(i32, i32), i32> = HashMap::with_capacity(nums.len());
        let mut op_count = usize::MAX;
        let mut is_one_found = false;
        let mut one_count = 0;

        for i in 0 .. nums.len() - 1 {
            let mut lhs = nums[i];

            if lhs == 1 {
                one_count += 1;
            }

            for j in i + 1 .. nums.len() {
                let mut rhs = nums[j];

                if rhs == 1 {
                    continue;
                }

                if op_count <= j - i {
                    break;
                }

                lhs = Self::gcd(lhs, nums[j]);

                if lhs == 1 {
                    op_count = op_count.min(j - i);
                    // println!("{op_count}");
                    break;
                }
            }
        }

        one_count += if nums[nums.len() - 1] == 1 {
            1
        } else {
            0
        };
        
        if one_count > 0 {
            return nums.len() as i32 - one_count as i32;
        }

        if op_count == usize::MAX {
            return -1;
        }

        op_count as i32 + nums.len() as i32 - 1
    }
}