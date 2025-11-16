impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut continual_count: i64 = 0;
        let mut result: i64 = 0;

        const MODULO: i64 = 1000000007;


        for item in s.into_bytes() {
            if item == b'1' {
                continual_count += 1;
                result += continual_count;
                result %= MODULO;
            } else {
                continual_count = 0;
            }
        }

        (result % MODULO) as i32
    }
}