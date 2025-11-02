impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let (mut left_id, mut right_id) = (0, heights.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut trapped_water = 0;

        while left_id < right_id {
            left_max = left_max.max(heights[left_id]);
            right_max = right_max.max(heights[right_id]);

            if left_max <= right_max {
                trapped_water += left_max - heights[left_id];
                left_id += 1;
            } else {
                trapped_water += right_max - heights[right_id];
                right_id -= 1;
            }
        }
        trapped_water
    }
}