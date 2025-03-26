impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut output_list: Vec<Vec<i32>> = Vec::new();

        for i in 0 .. nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left_id = i + 1;
            let mut right_id = nums.len() - 1;


            while left_id < right_id {
                let sum = nums[i] + nums[left_id] + nums[right_id];

                match sum {
                    sum if sum > 0 => right_id -= 1,
                    sum if sum < 0 => left_id += 1,
                    _ => {
                        output_list.push(vec![nums[i], nums[left_id], nums[right_id]]);
                        
                        while left_id < right_id && nums[left_id] == nums[left_id + 1] {
                            left_id += 1;
                        }

                        while left_id < right_id && nums[right_id] == nums[right_id - 1] {
                            right_id -= 1;
                        }
    
                        left_id += 1;
                        right_id -= 1;
                    }
                }
            }
        }
        output_list

    }
}