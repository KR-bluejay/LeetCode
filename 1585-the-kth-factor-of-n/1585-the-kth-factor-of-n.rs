impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut factor_count = 0;
        for i in 1 ..= n {
            if n % i == 0 {
                factor_count += 1;
            }
            if factor_count == k {
                return i;
            }
        }
        -1
    }
}