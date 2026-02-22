impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut max_dist = 0;
        let mut dist = 0;

        for t in format!("{:b}", n).into_bytes().into_iter() {
            if t == b'1' {
                max_dist = max_dist.max(dist);
                dist = 1;
            } else {
                dist += 1;
            }
        }

        max_dist
    }
}