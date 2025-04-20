impl Solution {
    fn lower_bound(nums: &Vec<i32>, target: i32, mut left: usize) -> i32 {
        if nums.len() == 0 || left >= nums.len() {
            return -1;
        }

        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if target <= nums[mid] {
                if mid == 0 {
                    break;
                }

                right = mid - 1;
            } else {
                if mid + 1 == nums.len() {
                    break;
                }

                left = mid + 1;
            }
        }

        left as i32
    }
    fn upper_bound(nums: &Vec<i32>, target: i32, mut left: usize) -> i32 {
        if nums.len() == 0 || left >= nums.len() {
            return -1;
        }

        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if target >= nums[mid] {
                if mid + 1 == nums.len() {
                    break;
                }
                left = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                right = mid - 1;

            }
        }

        right as i32
    }
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 1 && nums[0] == target {
            return vec![0, 0];
        }

        let start_id = Self::lower_bound(&nums, target, 0);

        if start_id == -1 || nums[start_id as usize] != target {
            return vec![-1, -1];
        }


        let mut end_id = Self::upper_bound(&nums, target, start_id.clone() as usize);



        return vec![start_id, end_id];
    }
}