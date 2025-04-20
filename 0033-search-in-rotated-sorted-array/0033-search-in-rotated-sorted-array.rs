impl Solution {
    fn b_search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left_id = 0;
        let mut right_id = nums.len() - 1;

        while 0 <= left_id && left_id < nums.len() && 0 <= right_id && right_id < nums.len() && left_id <= right_id {
            let mid_id: usize = left_id + ((right_id - left_id) / 2);

            let is_left_sorted: bool = nums[left_id] <= nums[mid_id];
            let is_right_sorted: bool = nums[mid_id] <= nums[right_id];

            if nums[mid_id] == target {
                return mid_id as i32;
            }

            if is_left_sorted && nums[left_id] <= target && target < nums[mid_id] {
                right_id = mid_id - 1;
                
                continue;
            }
            
            if is_right_sorted && nums[mid_id] < target && target <= nums[right_id] {
                left_id = mid_id + 1;
                continue;
            }

            if is_left_sorted && !(nums[left_id] <= target && target < nums[mid_id]) {
                left_id = mid_id + 1;

                continue;
            }
            right_id = mid_id - 1;

        }

        -1
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Self::b_search(&nums, target)
    }
}