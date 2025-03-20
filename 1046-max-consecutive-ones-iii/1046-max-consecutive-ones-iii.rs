use std::cmp;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left_index: i32 = 0;
        let mut init_index: i32 = 0;

        let mut right_index: i32 = 0;
        let mut flag_list: Vec<i32> = Vec::with_capacity(k as usize);


        for i in 0 .. nums.len() {
            if k == 0 {
                break;
            }

            if nums[i] == 0 {
                flag_list.push(i as i32);
            }
            init_index = i as i32;

            if flag_list.len() == k as usize {
                break;
            }
        }

        

        let mut max_sum: i32 = if init_index == 0 && k == 0 {
            0
        } else { 
            (init_index - left_index + 1)
        };



        if init_index == 0 && max_sum == 0 {
            init_index = -1;
        }

        for right_index in (init_index + 1) as usize .. nums.len() {
            if nums[right_index] == 0 && k == 0 {
                left_index = right_index as i32 + 1;
                continue;
            } else if nums[right_index] == 0 {
                left_index = (flag_list[0] + 1);
                flag_list[0] = right_index as i32;

                flag_list.sort();
            }

            let cur_sum = right_index as i32 - left_index + 1;

            println!("cur: {cur_sum}");
            max_sum = cmp::max(cur_sum, max_sum);
        }

        max_sum as i32
    }
}