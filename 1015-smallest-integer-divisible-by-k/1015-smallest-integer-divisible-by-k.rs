impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut remainder = 0;

        for id in 1 ..= k {
            remainder = (remainder * 10 + 1) % k;

            if remainder == 0 {
                return id;
            }
        }

        -1
    }
}