use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();


        for (num_pos, num_item) in nums.iter().enumerate() {
            let diff_num = num_map.get(&(target - num_item));


            match diff_num {
                Some(item) => {
                    println!("{item}"); 
                    return vec![*item, num_pos as i32];
                    
                },
                None => {
                    num_map.insert(*num_item, num_pos as i32);
                    continue;
                },
            }
        }

        return vec![]
    }
}