impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums[0] >= nums[1] {
            return false;
        }

        let mut inc_id = 0;
        let mut dec_id = 0;

        for id in 1 .. nums.len() - 1 {
            if nums[id] == nums[id + 1] {
                return false;
            }

            if nums[id] > nums[id + 1] {
                dec_id = id;
                
                break;
            }
        }

        for id in dec_id .. nums.len() - 1 {
            if nums[id] == nums[id + 1] {
                return false;
            }

            if nums[id] < nums[id + 1] {
                inc_id = id;
                
                break;
            }
        }

        for id in inc_id .. nums.len() - 1 {
            if nums[id] >= nums[id + 1] {
                return false;
            }
        }

        dec_id < inc_id
    }
}