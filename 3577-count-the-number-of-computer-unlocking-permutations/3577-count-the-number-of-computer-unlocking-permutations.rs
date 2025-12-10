impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        if complexity.iter().skip(1).any(|&v| complexity[0] >= v) {
            return 0;
        }

        let mut result: usize = 1;
        const MODULO: usize = 1_000_000_007;

        for i in 2 .. complexity.len() {
            result *= i;
            result %= MODULO
        }

        result as i32
    }
}