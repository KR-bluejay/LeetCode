use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Block {
    row_id: usize,
    col_id: usize,
    height: i32
}

impl Ord for Block {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        other.height.cmp(&self.height)
    }
}

impl PartialOrd for Block {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    #[inline(always)]
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let max_row_id = height_map.len() - 1;
        let max_col_id = height_map[0].len() - 1;

        let mut water_amount = 0;

        let mut block_queue: BinaryHeap<Block> 
            = BinaryHeap::with_capacity((max_row_id + 1) * (max_col_id + 1));
        let mut block_visit: Vec<Vec<bool>> = vec![vec![false; max_col_id + 1]; max_row_id + 1];

        for row_id in 0 ..= max_row_id {
            block_visit[row_id][0] = true;
            block_queue.push(Block {
                row_id,
                col_id: 0,
                height: height_map[row_id][0]
            });

            block_visit[row_id][max_col_id] = true;
            block_queue.push(Block {
                row_id,
                col_id: max_col_id,
                height: height_map[row_id][max_col_id]
            });
        }

        for col_id in 1 .. max_col_id {
            block_visit[0][col_id] = true;
            block_queue.push(Block {
                row_id: 0,
                col_id,
                height: height_map[0][col_id]
            });

            block_visit[max_row_id][col_id] = true;
            block_queue.push(Block {
                row_id: max_row_id,
                col_id,
                height: height_map[max_row_id][col_id]
            });
        }


        while let Some(block) = block_queue.pop() {
            let Block { row_id, col_id, height } = block;
            let next_pos = [
                (row_id, col_id.wrapping_sub(1)), 
                (row_id, col_id + 1), 
                (row_id.wrapping_sub(1), col_id), 
                (row_id + 1, col_id)
            ];

            for (&(next_row_id, next_col_id))in next_pos.iter() {
                if max_row_id < next_row_id 
                || max_col_id < next_col_id 
                || block_visit[next_row_id][next_col_id] {
                    continue;
                }
                let next_height = height_map[next_row_id][next_col_id];

                water_amount += (height - next_height).max(0);

                block_visit[next_row_id][next_col_id] = true;
                block_queue.push(Block {
                    row_id: next_row_id,
                    col_id: next_col_id,
                    height: height.max(next_height)
                });
            }
        }

        water_amount

    }
}