impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable_by(|left, right| right.cmp(&left));

        let k = k as usize;

        happiness[0 .. k].into_iter().enumerate().fold(0, |acc, (id, &val)| acc + (val as i64 - id as i64).max(0))
    }
}