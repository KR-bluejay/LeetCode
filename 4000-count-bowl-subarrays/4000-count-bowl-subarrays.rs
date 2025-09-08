impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut bowl_stack: Vec<usize> = Vec::with_capacity(nums.len());
        let mut bowl_count: i64 = 0;
        let mut bowl_pos: Vec<(usize, usize)> = Vec::with_capacity(nums.len());


        for num_id in 0 .. nums.len() {
            let num_val = nums[num_id];

            while let Some(bowl_num_id) = bowl_stack.pop() {
                let bowl_num_val = nums[bowl_num_id];

                if num_val < bowl_num_val {
                    bowl_stack.push(bowl_num_id);

                    break;
                }

                bowl_pos.push((num_id, bowl_num_id));
            }

            if let Some(bowl_num_id) = bowl_stack.last() {
                bowl_pos.push((num_id, *bowl_num_id));
            }

            bowl_stack.push(num_id);
        }

        

        for bowl_pos_item in bowl_pos.iter() {
            let left_id = bowl_pos_item.0;
            let right_id = bowl_pos_item.1;

            if right_id - left_id + 1 >= 3 {
                bowl_count += 1;
            }
        }


        bowl_count
    }
}