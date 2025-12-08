impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        // a^2 + b^2 = c^2
        // (a + b)^2 - 2ab = c^2
        let mut count = 0;

        for k in 5 ..= n {
            for i in 1 .. n {
                for j in 1 .. n {

                    if (i * i) + (j * j) == (k * k) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}