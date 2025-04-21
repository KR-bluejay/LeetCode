impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut ret = 0;
        let mut nums: Vec<(i32, i32)> = Vec::with_capacity(differences.len() + 1);

        let mut prev_lower = lower.clone();
        let mut prev_upper = upper.clone();


        for &diff_item in differences.iter()  {
            prev_lower = lower.max(diff_item + prev_lower);
            prev_upper = upper.min(diff_item + prev_upper);
            
            if prev_upper - prev_lower < 0 {
                return 0;
            }
        }

        prev_upper - prev_lower + 1
    }
}