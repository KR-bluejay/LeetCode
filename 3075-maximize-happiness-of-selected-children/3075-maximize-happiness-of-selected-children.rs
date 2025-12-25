impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();

        let mut result = 0;
        let mut result2 = 0;

        let k = k as usize;

        for id in (happiness.len() - k) .. happiness.len() {
            result += (happiness[id] as i64 + (happiness.len() - k - id) as i64).max(0);
        }

        for id in ((happiness.len() - k) .. happiness.len()).rev() {
            result2 += (happiness[id] as i64 - (happiness.len() - 1 - id) as i64).max(0);
        }


        result.max(result2)
    }
}