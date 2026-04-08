impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MODULO: i64 = 1_000_000_007;

        for query in queries.into_iter() {
            let (l, r, k, v) = (
                query[0] as usize, 
                query[1] as usize, 
                query[2] as usize, 
                query[3] as i64
            );

            for id in (l ..= r).step_by(k) {
                nums[id] = ((nums[id] as i64 * v) % MODULO) as i32;
            }
        }

        nums.into_iter().fold(0, |acc, n| acc ^ n)
    }
}