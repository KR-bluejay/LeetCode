impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> usize {
        while b > 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a as usize
    }
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut factor_count: i32 = 0;
        let mut gcd = Self::gcd(a, b);

        for i in 1 ..= gcd {
            if gcd % i == 0 {
                factor_count += 1;
            }
        }

        factor_count
    }
}