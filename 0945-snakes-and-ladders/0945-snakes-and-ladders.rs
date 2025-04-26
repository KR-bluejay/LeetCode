use std::collections::{ BinaryHeap };
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut history: Vec<i32> = vec![-1; board.len().pow(2) + 1];

        let mut columns: Vec<usize> = 
            Vec::with_capacity(board.len());
        let mut cells: Vec<(usize, usize)> = 
            vec![(0, 0); board.len().pow(2) + 1];

        let mut cell_id = 1;

        for i in 0 .. board.len() {
            columns.push(i);
        }


        for row_id in (0 .. board.len()).rev() {
            for &column_id in columns.iter() {
                cells[cell_id] = (row_id, column_id);
                cell_id += 1
            }
            columns.reverse();
        }

        history[1] = 0;

        let mut board_queue: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        
        board_queue.push((0, 1));

        while !board_queue.is_empty() {
            let (dist, cur_pos) = board_queue.pop().unwrap();

            if history[cur_pos] != dist {
                continue;
            }

            for next_pos in (cur_pos + 1) ..= (cur_pos + 6).min(board.len().pow(2)) {
                let (next_row, next_col) = cells[next_pos];
                let dest = if board[next_row][next_col] != -1 {
                    board[next_row][next_col] as usize
                } else {
                    next_pos
                };

                if history[dest] == -1 || history[dest] > dist + 1 {
                    history[dest] = dist + 1;
                    board_queue.push((dist + 1, dest));
                }
            }
        }


        return history[history.len() - 1];
    }
}