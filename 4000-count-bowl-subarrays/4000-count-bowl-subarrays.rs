impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut bowl_stack: Vec<usize> = Vec::with_capacity(nums.len());
        let mut bowl_count: i64 = 0;


        for num_id in 0 .. nums.len() {
            let num_val = nums[num_id];

            while let Some(bowl_num_id) = bowl_stack.pop() {
                let bowl_num_val = nums[bowl_num_id];

                if num_val < bowl_num_val {
                    bowl_stack.push(bowl_num_id);

                    break;
                }
                bowl_count += 1;
            }

            if let Some(bowl_num_id) = bowl_stack.last() {
                bowl_count += 1;
            }

            bowl_stack.push(num_id);
        }


        

        bowl_count - (nums.len() - 1) as i64
    }
}