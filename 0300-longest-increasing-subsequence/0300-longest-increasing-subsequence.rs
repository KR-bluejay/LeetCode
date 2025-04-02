impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut sub_nums: Vec<i32> = Vec::new();

        for &num_item in nums.iter() {
            if *sub_nums.last().unwrap_or(&i32::MIN) < num_item {
                sub_nums.push(num_item);

                continue;
            }
            let update_id = match sub_nums.binary_search(&num_item) {
                Ok(v) | Err(v) => v,
            };

            sub_nums[update_id] = num_item;
        }

        sub_nums.len() as i32
    }
}