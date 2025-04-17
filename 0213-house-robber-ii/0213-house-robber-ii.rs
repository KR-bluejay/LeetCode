impl Solution {
    fn rob_house(house_id: usize, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if house_id >= nums.len() {
            return 0;
        }

        if cache[house_id] != -1 {
            return cache[house_id]
        }

        // Skip Current House
        let skip_reward = Self::rob_house(house_id + 1, nums, cache);
        let rob_reward = nums[house_id] + Self::rob_house(house_id + 2, nums, cache).max(Self::rob_house(house_id + 3, nums, cache));

        cache[house_id] = skip_reward.max(rob_reward);
        cache[house_id]
    }
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        let mut cache: Vec<i32> = vec![-1; nums.len()];
        let first_case = Self::rob_house(1, &nums, &mut cache);
        
        cache.fill(-1);

        let last_id = nums.len() - 1;

        if last_id != 0 {
            nums[last_id] = 0;
        }
        let second_case =  Self::rob_house(0, &nums, &mut cache);

        first_case.max(second_case)
    }
}