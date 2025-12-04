impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let directions: Vec<u8> = directions.into_bytes();
        
        let mut collsion_count = 0;
        // let mut collison_stack: VecDeque<u8> = VecDeque::with_capacity(direction.len());
        let mut left_count = 0;
        let mut stay_count = 0;
        let mut right_count = 0;

        for direction in directions.iter() {
            match direction {
                b'L' => {
                    left_count = 1;
                    if right_count > 0 {
                        collsion_count += left_count + right_count;
                        left_count = 0;
                        stay_count = 1;
                        right_count = 0;
                    } else if stay_count > 0 {
                        collsion_count += left_count;
                        left_count = 0;
                        stay_count = 1;
                    } else {
                        left_count = 1;
                    }
                },
                b'S' => {
                    if right_count > 0 {
                        collsion_count += right_count;
                    }
                    right_count = 0;
                    stay_count = 1;
                },
                b'R' => {
                    right_count += 1;
                },
                _ => {}
            }
        }

        collsion_count
    }
}