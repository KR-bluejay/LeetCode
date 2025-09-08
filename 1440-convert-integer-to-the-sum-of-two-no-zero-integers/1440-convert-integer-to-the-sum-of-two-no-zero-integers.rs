impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut b = n - 1;
        let mut a = n - b;

        while a.to_string().contains("0") || b.to_string().contains("0") {
            b -= 1;
            a += 1;
        }

        vec![a, b]
    }
}