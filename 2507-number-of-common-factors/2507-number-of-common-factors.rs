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
        let mut factor_count = 1;
        let mut gcd = Self::gcd(a, b);

        let mut sieve: Vec<usize> = vec![1; gcd + 1];

        for i in 2 ..= gcd {
            if sieve[i] != 1 {
                continue;
            }
            for j in 1 ..= (gcd / i) {
                if sieve[i * j] != 1 {
                    continue;
                }
                sieve[i * j] = i;
            }
        }

        let prime_list: Vec<usize> = sieve.iter().enumerate().filter(|(i, v)| **v > 1 && *i == **v).map(|(_, v)| *v).collect();


        for &prime_item in prime_list.iter() {
            if gcd <= 1 || prime_item > gcd {
                break;
            }
            let mut count = 0;

            while gcd > 1 && prime_item <= gcd && gcd % prime_item == 0 {
                count += 1;
                gcd /= prime_item;
            }
            count += 1;
            

            factor_count *= count;
        }
        factor_count as i32
    }
}