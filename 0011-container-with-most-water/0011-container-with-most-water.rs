impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_idx = 0;
        let mut right_idx = height.len() - 1;
        let mut max_size = 0;

        while left_idx < right_idx {
            let left_len = height[left_idx];
            let right_len = height[right_idx];
            let distance = right_idx - left_idx;
            let height = left_len.min(right_len);

            let cur_size = distance as i32 * height;
            max_size = max_size.max(cur_size);
            

            if left_len < right_len {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }
        max_size
    }
}