impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut num1_sum: i64 = nums1.iter().map(|&v| v as i64).sum::<i64>();
        let mut num2_sum: i64 = nums2.iter().map(|&v| v as i64).sum::<i64>();

        let mut num1_wild_count: i64 = nums1.iter().filter(|&v| *v == 0).count() as i64;
        let mut num2_wild_count: i64 = nums2.iter().filter(|&v| *v == 0).count() as i64;



        if num1_wild_count > 0 {
            num1_sum += num1_wild_count - 1;
            num1_wild_count = 1;
        }

        if num2_wild_count > 0 {
            num2_sum += num2_wild_count - 1;
            num2_wild_count = 1;
        }

        if num1_sum == num2_sum && num1_wild_count == 0 && num2_wild_count == 0 {
            return num1_sum as i64;
        }


        if num1_sum == num2_sum && (num1_wild_count == 0 || num2_wild_count == 0) {
            return -1;
        }

        if (num1_sum < num2_sum && num1_wild_count == 0) || (num2_sum < num1_sum && num2_wild_count == 0) {
            return -1;
        }

        let diff_num = (num2_sum - num1_sum).abs() as i64;


        if num1_sum < num2_sum && num2_wild_count == 0 {
            return num2_sum as i64;
        } else if num2_sum < num1_sum && num1_wild_count == 0 {
            return num1_sum as i64;
        }

        (num1_sum.max(num2_sum) + 1) as i64
    }
}