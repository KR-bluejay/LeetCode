use std::collections::HashSet;

impl Solution {
    pub fn count_unguarded(
        m: i32, 
        n: i32, 
        guards: Vec<Vec<i32>>, 
        walls: Vec<Vec<i32>>
    ) -> i32 {
        let max_row_id = m as usize - 1;
        let max_col_id = n as usize - 1;

        let mut guarded: Vec<Vec<u8>>
            = vec![vec![0; max_col_id + 1]; max_row_id + 1];

        for guard in guards {
            let row_id = guard[0] as usize;
            let col_id = guard[1] as usize;

            guarded[row_id][col_id] = 2;
        }
        
        for wall in walls {
            let row_id = wall[0] as usize;
            let col_id = wall[1] as usize;

            guarded[row_id][col_id] = 1;
        }

        // 0: None, 1: Wall, 2: Guard, 3: AlreadyGuarded
        // Top -> Bottom
        for row_id in 0 ..= max_row_id {
            let mut left_id = 0;
            let mut right_id = max_col_id;

            let mut left_guard = false;
            let mut right_guard = false;

            while left_id <= max_col_id {
                match guarded[row_id][left_id] {
                    0 if left_guard => guarded[row_id][left_id] = 3,
                    1 => left_guard = false,
                    2 => left_guard = true,
                    _ => {},
                }
                match guarded[row_id][right_id] {
                    0 if right_guard => guarded[row_id][right_id] = 3,
                    1 => right_guard = false,
                    2 => right_guard = true,
                    _ => {},
                }

                left_id += 1;
                right_id -= 1;
            }
        }
        
        // 0: None, 1: Wall, 2: Guard
        // Top -> Bottom
        for col_id in 0 ..= max_col_id {
            let mut top_id = 0;
            let mut bottom_id = max_row_id;

            let mut top_guard = false;
            let mut bottom_guard = false;

            while top_id <= max_row_id {
                match guarded[top_id][col_id] {
                    0 if top_guard => guarded[top_id][col_id] = 3,
                    1 => top_guard = false,
                    2 => top_guard = true,
                    _ => {},
                }

                match guarded[bottom_id][col_id] {
                    0 if bottom_guard => guarded[bottom_id][col_id] = 3,
                    1 => bottom_guard = false,
                    2 => bottom_guard = true,
                    _ => {},
                }
                top_id += 1;
                bottom_id -= 1;
            }
        }

        guarded.into_iter()
            .map(|row| row.into_iter().filter(|&c| c == 0).count() as i32)
            .sum()
    }
}