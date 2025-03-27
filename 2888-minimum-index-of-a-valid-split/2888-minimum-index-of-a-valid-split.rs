
impl Solution {
    fn get_dominant_element(nums: &Vec<i32>) -> i32 {
        let mut candidate_num = nums[0];
        let mut candidate_count = 1;
        
        for num_item in nums.iter().skip(1) {
            if candidate_num == *num_item {
                candidate_count += 1;

                continue;
            }

            if candidate_count == 0 {
                candidate_num = *num_item;
                candidate_count = 1;
                
                continue;
            }
            
            candidate_count -= 1;
        }

        candidate_num
    }
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let dominant_num = Self::get_dominant_element(&nums);
        let mut left_count = 0;
        let mut right_count = nums.iter().filter(|num_item| **num_item == dominant_num).count();

        for (num_index, &num_item) in nums.iter().enumerate() {
            if num_item != dominant_num {
                continue;
            }

            left_count += 1;
            right_count -= 1;

            let is_left_dominant = left_count * 2 > (num_index + 1);
            let is_right_dominant = right_count * 2 > (nums.len() - num_index - 1);

            if is_left_dominant && is_right_dominant {
                return num_index as i32;
            }
        }
        -1
    }
}