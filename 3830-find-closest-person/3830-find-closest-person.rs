use std::cmp::Ordering;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let x_dist = (x - z).abs();
        let y_dist = (y - z).abs();

        match x_dist.cmp(&y_dist) {
            Ordering::Less => {
                1
            },
            Ordering::Equal => {
                0
            },
            Ordering::Greater => {
                2
            }
        }
    }
}