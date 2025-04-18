impl Solution {
    fn get_mod(mut num: i64, mut pow_count: i64) -> i64 {
        let mut res: i64 = 1;

        while pow_count > 0 {
            if pow_count % 2 == 1 {
                res = res * num % 1000000007;
                pow_count -= 1;
            }
            num = num * num % 1000000007;
            pow_count /= 2;
        }
        res % 1000000007
    }
    pub fn count_good_numbers(n: i64) -> i32 {
        (Self::get_mod(5, (n + 1) / 2) * Self::get_mod(4, n / 2)  % 1000000007) as i32
    }
}