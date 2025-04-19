impl Solution {
    fn find_lower_boundary(nums: &Vec<i32>, mut left_id: usize, search_num: i32) -> usize {
        let mut right_id: usize = nums.len() - 1;

        while left_id <= right_id {
            let mid_id: usize = left_id + ((right_id - left_id) / 2);

            if nums[mid_id] >= search_num {
                right_id = mid_id - 1;
            } else {
                left_id = mid_id + 1;
            }
        }
        left_id
    }
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();

        let mut fair_count: i64 = 0;

        for num_id in 0 .. nums.len() - 1 {
            let num_item = nums[num_id];

            let lower_bounary_id = Self::find_lower_boundary(
                &nums, 
                num_id + 1, 
                lower - num_item
            );

            let upper_bounary_id = Self::find_lower_boundary(
                &nums, 
                num_id + 1, 
                upper - num_item + 1
            );

            fair_count += (upper_bounary_id - lower_bounary_id) as i64;
        }

        fair_count
    }
}