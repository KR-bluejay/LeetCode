impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const PRIME_BIT: u32 = 1 << 2 
            | 1 << 3 
            | 1 << 5 
            | 1 << 7 
            | 1 << 11 
            | 1 << 13 
            | 1 << 17 
            | 1 << 19 
            | 1 << 23
            | 1 << 29 
            | 1 << 31;

        (left ..= right).map(|mid| (PRIME_BIT >> mid.count_ones()) & 1)
            .sum::<u32>() as i32
    }
}