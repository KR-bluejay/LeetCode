use std::collections::{ HashSet };

impl Solution {
    fn backtrack(
        row_id: i32, 
        board_size: i32,
        cur_board: &mut Vec<String>,
        column_set: &mut HashSet<i32>, 
        diagonal_set: &mut HashSet<i32>, 
        anti_diagonal_set: &mut HashSet<i32>,
        solution_list: &mut Vec<Vec<String>>,
    ) {
        if row_id == board_size {
            println!("{cur_board:?}");
            solution_list.push(cur_board.to_vec());
        }

        for column_id in  0 .. board_size {
            let is_not_column = column_set.contains(&column_id);
            let is_not_diagonal = diagonal_set.contains(&(row_id + column_id));
            let is_not_anti_diagonal = anti_diagonal_set.contains(&(row_id - column_id));

            if is_not_column || is_not_diagonal || is_not_anti_diagonal {
                continue;
            }

            let cur = ".".repeat(column_id as usize) + "Q" + &".".repeat((board_size - column_id - 1) as usize);

            cur_board.push(cur);
            column_set.insert(column_id);
            diagonal_set.insert(row_id + column_id);
            anti_diagonal_set.insert(row_id - column_id);


            Self::backtrack(row_id + 1, board_size, cur_board, column_set, diagonal_set, anti_diagonal_set, solution_list);

            cur_board.pop();
            column_set.remove(&column_id);
            diagonal_set.remove(&(row_id + column_id));
            anti_diagonal_set.remove(&(row_id - column_id));
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solution_list: Vec<Vec<String>> = Vec::with_capacity(n as usize);
        
        let mut cur_board: Vec<String> = Vec::with_capacity(n as usize);
        let mut column_set: HashSet<i32> = HashSet::with_capacity(n as usize);
        let mut diagonal_set: HashSet<i32> = HashSet::with_capacity(n as usize);
        let mut anti_diagonal_set: HashSet<i32> = HashSet::with_capacity(n as usize);


        Self::backtrack(0, n, &mut cur_board, &mut column_set, &mut diagonal_set, &mut anti_diagonal_set, &mut solution_list);

        solution_list
    }
}