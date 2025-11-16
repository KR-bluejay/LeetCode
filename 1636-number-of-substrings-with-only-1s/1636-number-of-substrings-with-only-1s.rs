impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut continual_count: u32 = 0;
        let mut result: u32 = 0;

        const MODULO: u32 = 1000000007;


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