use std::collections::VecDeque;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let (min_jump, max_jump) = (min_jump as usize, max_jump as usize);
        let offset = max_jump - min_jump;
        
        let tile_len = s.len();
        
        let mut max_id = 0;
        let mut reach_queue = VecDeque::with_capacity(offset + 1);

        reach_queue.push_back(min_jump);

        for (id, tile) in s.into_bytes().into_iter().enumerate().skip(1) {
            if tile == b'1' {
                continue;
            }

            while let Some(&front_id) = reach_queue.front() 
            && front_id + offset < id {
                reach_queue.pop_front();
            }

            if let Some(&front_id) = reach_queue.front() {
                if front_id <= id {
                    max_id = id;
                    reach_queue.push_back(id + min_jump);
                }
            } else {
                break;
            }
        }

        max_id + 1 == tile_len
    }
}