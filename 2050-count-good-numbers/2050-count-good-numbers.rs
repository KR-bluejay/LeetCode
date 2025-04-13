impl Solution {
    
    fn count_number(mut a: i64, mut b: i64) -> i64 {
        let mut ret: i64 = 1;

        while b > 0  {
            if b % 2 == 1 {
                ret = (ret * a) % (10_i64.pow(9) + 7);
            }

            a = a * a % (10_i64.pow(9) + 7);
            b /= 2;
        }

        ret
    }
    pub fn count_good_numbers(n: i64) -> i32 {
        (Self::count_number(5, (n + 1) / 2) * Self::count_number(4, n / 2) % (10_i64.pow(9) + 7)) as i32
    }
    
}