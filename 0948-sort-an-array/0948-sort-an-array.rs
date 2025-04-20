impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let sub_array_size = nums.len() / 2;

        if nums.len() <= 1 {
            return nums;
        }

        let first_arr = Self::sort_array(nums[0 .. sub_array_size].to_vec());
        let second_arr = Self::sort_array(nums[sub_array_size .. nums.len()].to_vec());

        Self::merge_sort(first_arr, second_arr)
    }
    fn merge_sort(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut merged_nums: Vec<i32> = Vec::with_capacity(nums1.len() + nums2.len());

        let mut num1_id = 0;
        let mut num2_id = 0;

        let mut merged_id = 0;

        while num1_id < nums1.len() && num2_id < nums2.len() {
            if nums1[num1_id] < nums2[num2_id] {
                merged_nums.push(nums1[num1_id]);
                num1_id += 1;
            } else {
                merged_nums.push(nums2[num2_id]);
                num2_id += 1;
            }
        }

        while num1_id < nums1.len() {
            merged_nums.push(nums1[num1_id]);
            
            num1_id += 1;
        }

        while num2_id < nums2.len() {
            merged_nums.push(nums2[num2_id]);
            
            num2_id += 1;
        }

        merged_nums
    }
}