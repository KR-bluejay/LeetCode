impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let mut water_volume = 0;
        let mut stack_ids: Vec<usize> = Vec::with_capacity(heights.len());

        for (right_id, &right_val) in heights.iter().enumerate() {
            let mut prev_height = 0;

            while let Some(&left_id) = stack_ids.last() 
            && heights[left_id] <= right_val {
                let width = (right_id - left_id) as i32 - 1;
                let height = heights[left_id] - prev_height;

                water_volume += width * height;
                prev_height = heights[left_id];
                
                stack_ids.pop();
            }

            if 0 < stack_ids.len() && prev_height < right_val {
                let width = (right_id - stack_ids.last().unwrap()) as i32 - 1;
                let height = right_val - prev_height;

                water_volume += width * height;
            }
            stack_ids.push(right_id);
        }
        water_volume
    }
}