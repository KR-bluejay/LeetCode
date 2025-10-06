use std::collections::{ BinaryHeap };
use std::cmp::{ Ordering };

#[derive(Clone, Debug, Eq, PartialEq)]
struct Block {
    cur_time: i32,
    row_id: usize,
    col_id: usize,
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cur_time.cmp(&other.cur_time)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut min_time = i32::MAX;
        let mut min_grid = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

        let max_row_id = grid.len() - 1;
        let max_col_id = grid[0].len() - 1;
        
        let positions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut block_heap: BinaryHeap<Block> = BinaryHeap::with_capacity(grid.len());

        block_heap.push(Block {
            row_id: 0,
            col_id: 0,
            cur_time: grid[0][0]
        });

        while let Some(target_block) = block_heap.pop() {
            let Block { row_id, col_id, cur_time } = target_block;
            
            if min_grid[row_id][col_id] <= cur_time 
            || min_grid[max_row_id][max_col_id] <= cur_time {
                continue;
            }

            min_grid[row_id][col_id] = cur_time;

            for (dx, dy) in positions.iter() {
                let next_row_id = (row_id as i32 + dy) as usize;
                let next_col_id = (col_id as i32 + dx) as usize;

                if max_row_id < next_row_id 
                || max_col_id < next_col_id {
                    continue;
                }
                let next_time = cur_time.max(grid[next_row_id][next_col_id]);

                if min_grid[max_row_id][max_col_id] <= next_time {
                    continue;
                }

                block_heap.push(Block {
                    row_id: next_row_id,
                    col_id: next_col_id,
                    cur_time: next_time,
                });
            }
        }

        min_grid[max_row_id][max_col_id]
    }
}