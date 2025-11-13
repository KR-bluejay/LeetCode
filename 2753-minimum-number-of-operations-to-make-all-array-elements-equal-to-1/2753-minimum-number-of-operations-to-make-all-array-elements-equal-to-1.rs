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
        let mut op_count = usize::MAX;

        for i in 0 .. nums.len() - 1 {
            let mut lhs = nums[i];

            if lhs == 1 {
                return nums.len() as i32  - nums.iter().skip(i).filter(|v| **v == 1).count() as i32;
            }

            for j in i + 1 .. nums.len() {
                let mut rhs = nums[j];

                if rhs == 1 {
                    return nums.len() as i32  - nums.iter().skip(j).filter(|v| **v == 1).count() as i32;
                }

                if op_count <= j - i {
                    break;
                }

                lhs = Self::gcd(lhs, nums[j]);

                if lhs == 1 {
                    op_count = op_count.min(j - i);
                    break;
                }
            }
        }

        if op_count == usize::MAX {
            return -1;
        }

        op_count as i32 + nums.len() as i32 - 1
    }
}