impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        (1 << (i32::BITS - n.leading_zeros())) - 1
    }
}