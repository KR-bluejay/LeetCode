impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.extend(nums2);
        nums1.sort_unstable();

        (nums1[nums1.len() / 2] as f64 + nums1[(nums1.len() - 1) / 2] as f64) / 2.0
    }
}