impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut res = 0;

        for i in 0 .. 31 {
            res += (n & 1);
            n >>= 1;
        }
        res
    }
}