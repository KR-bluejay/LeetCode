use std::collections::{ HashSet };

impl Solution {
    fn backtrack(
        row_id: i32,
        board_size: i32,
        column_set: &mut HashSet<i32>, 
        diagonal_set: &mut HashSet<i32>, 
        anti_diagonal_set: &mut HashSet<i32>
    ) -> i32 {
        if row_id == board_size {
            return 1;
        }

        let mut case_count = 0;

        for column_id in 0 .. board_size {
            let is_column = !column_set.contains(&column_id);
            let is_diagonal = !diagonal_set.contains(&(row_id + column_id));
            let is_anti_diagonal = !anti_diagonal_set.contains(&(row_id - column_id));

            if !(is_column && is_diagonal && is_anti_diagonal) {
                continue;
            }

            column_set.insert(column_id);
            diagonal_set.insert(row_id + column_id);
            anti_diagonal_set.insert(row_id - column_id);

            case_count += Self::backtrack(row_id + 1, board_size, column_set, diagonal_set, anti_diagonal_set);

            column_set.remove(&column_id);
            diagonal_set.remove(&(row_id + column_id));
            anti_diagonal_set.remove(&(row_id - column_id));
        }

        case_count
    }
    pub fn total_n_queens(n: i32) -> i32 {
        let mut diagonal_set: HashSet<i32> = HashSet::with_capacity(n as usize);
        let mut anti_diagonal_set: HashSet<i32> = HashSet::with_capacity(n as usize);
        let mut column_set: HashSet<i32> = HashSet::with_capacity(n as usize);

        Self::backtrack(0, n, &mut column_set, &mut diagonal_set, &mut anti_diagonal_set)
    }
}