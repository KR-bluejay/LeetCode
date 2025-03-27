use std::collections::{ HashSet };

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_set: HashSet<u32> = HashSet::with_capacity(9);
        let mut column_set: HashSet<u32> = HashSet::with_capacity(9);
        let mut grid_set: HashSet<u32> = HashSet::with_capacity(9);

        for i in 0 .. 9 {
            for j in 0 .. 9 {
                if board[i][j] != '.' {
                    let row_num = board[i][j].to_digit(10).unwrap_or(0);
                    
                    if !row_set.insert(row_num) {
                        return false;
                    }
                }

                if board[j][i] != '.' {
                    let column_num = board[j][i].to_digit(10).unwrap_or(0);
                    
                    if !column_set.insert(column_num) {
                        return false;
                    }
                }
            }
            row_set.clear();
            column_set.clear();
        }

        for i in 0 .. 3 {
            for h in 0 .. 3 {
                for j in i * 3 ..= i * 3 + 2 {
                    for k in h * 3 ..= h * 3 + 2 {
                        if board[j][k] != '.' {
                            let row_num = board[j][k].to_digit(10).unwrap_or(0);
                        
                            if !grid_set.insert(row_num) {
                                return false;
                            }
                        }
                    }
                }
                grid_set.clear();
            }
        }

        true    
    }
}