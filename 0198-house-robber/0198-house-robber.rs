impl Solution {
    fn dp(house_id: usize, nums: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
        if house_id >= nums.len()  {
            return 0;
        }

        if memo[house_id] != -1 {
            return memo[house_id];
        }

        let case_one = nums[house_id] + Self::dp(house_id + 2, nums, memo);
        let case_three = nums[house_id] + Self::dp(house_id + 3, nums, memo);
        let case_two = Self::dp(house_id + 1, nums, memo);



        memo[house_id] = case_one.max(case_two).max(case_three);

        memo[house_id]
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut memo: Vec<i32> = vec![-1; nums.len()];

        
        memo[nums.len() - 1] = nums[nums.len() - 1];

        if nums.len() >= 2 {
            memo[nums.len() - 2] = nums[nums.len() - 2];
        }

        match nums.len() {
            1 => return nums[0],
            2 => return nums[0].max(nums[1]),
            _ => {},
        }
        

        Self::dp(0, &nums, &mut memo);

        memo[0]
    }
}