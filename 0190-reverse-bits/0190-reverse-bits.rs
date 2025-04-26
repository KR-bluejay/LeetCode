impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut result: u32 = 0;

        for i in 0 .. 32 {
            result |= (x & 1) << (31 - i);
            x >>= 1;
        }
        result
    }
}