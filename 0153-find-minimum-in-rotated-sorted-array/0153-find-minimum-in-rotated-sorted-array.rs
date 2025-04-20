impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;


        while left <= right {
            let mid = left + (right - left) / 2;

            let is_left_sorted = nums[left] <= nums[mid];
            let is_right_sorted = nums[mid] <= nums[right];


            if is_left_sorted && is_right_sorted {
                return nums[left];
            }

            if is_right_sorted {
                if mid == 0 {
                    return 0;
                }

                if nums[mid] < nums[mid - 1] {
                    return nums[mid];
                }
                right = mid - 1;
            } else {
                if mid + 1 == nums.len() {
                    return nums[mid];
                }

                if nums[mid] > nums[mid + 1] {
                    return nums[mid + 1];
                }

                left = mid + 1;
            }
        }

        nums[left]
    }
}