impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let directions: Vec<u8> = directions.into_bytes();
        let opt_left_id: Option<usize> = directions.iter()
            .position(|&d| d == b'S' || d == b'R');
        let opt_right_id: Option<usize> = directions.iter()
            .rposition(|&d| d == b'S' || d == b'L');

        if let Some(left_id) = opt_left_id 
        && let Some(right_id) = opt_right_id 
        && left_id < right_id {
            directions[left_id ..= right_id].iter()
                .fold(0, |mut acc, &d| acc + (d != b'S') as i32)
        } else {
            0
        }
    }
}