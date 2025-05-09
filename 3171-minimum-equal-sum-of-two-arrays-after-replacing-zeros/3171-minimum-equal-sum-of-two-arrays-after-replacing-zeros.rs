impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut num1_sum: i64 = nums1.iter().map(|&v| v as i64).sum::<i64>();
        let mut num2_sum: i64 = nums2.iter().map(|&v| v as i64).sum::<i64>();

        let mut num1_wild_count: i64 = nums1.iter().filter(|&v| *v == 0).count() as i64;
        let mut num2_wild_count: i64 = nums2.iter().filter(|&v| *v == 0).count() as i64;



        num1_sum += num1_wild_count;
        num2_sum += num2_wild_count;


        if (num1_sum < num2_sum && num1_wild_count == 0) || (num2_sum < num1_sum && num2_wild_count == 0) {
            return -1;
        }


        num1_sum.max(num2_sum)
    }
}