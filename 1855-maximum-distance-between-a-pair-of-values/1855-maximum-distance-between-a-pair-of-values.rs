impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut num1_id = 0;
        let mut num2_id = 0;

        let mut max_dist = 0;

        while num1_id < nums1.len() && num2_id < nums2.len() {
            num2_id = num2_id.max(num1_id);

            while num2_id + 1 < nums2.len() 
            && nums1[num1_id] <= nums2[num2_id + 1] {
                num2_id += 1;
            }

            max_dist = max_dist.max(num2_id - num1_id);

            num1_id += 1;
        }

        max_dist as i32
    }
}