#[derive(Debug)]
struct Top3 {
    vals: [i32; 3],
}

impl Top3 {
    fn new() -> Self {
        Self {
            vals: [0; 3],
        }
    }
    
    fn insert(&mut self, val: i32) {
        if val > self.vals[0] {
            self.vals[2] = self.vals[1];
            self.vals[1] = self.vals[0];
            self.vals[0] = val;
        } else if val != self.vals[0] && val > self.vals[1] {
            self.vals[2] = self.vals[1];
            self.vals[1] = val;
        } else if val != self.vals[0] && val != self.vals[1] 
        && val > self.vals[2] {
            self.vals[2] = val;
        }
    }

    fn to_vec(self) -> Vec<i32> {
        self.vals.into_iter()
            .filter(|&v| v != 0)
            .collect()
    }
}

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut diagonals = vec![vec![0; grid[0].len()]; grid.len()];
        let mut anti_diagonals = vec![vec![0; grid[0].len()]; grid.len()];

        for row_id in 0 .. grid.len() {
            for col_id in 0 .. grid[0].len() {
                anti_diagonals[row_id][col_id] = grid[row_id][col_id];
                anti_diagonals[row_id][col_id] += if row_id > 0 && col_id > 0 {
                    anti_diagonals[row_id - 1][col_id - 1]
                } else {
                    0
                };

                diagonals[row_id][col_id] = grid[row_id][col_id];
                diagonals[row_id][col_id] += if row_id > 0 
                && col_id + 1 < grid[0].len() {
                    diagonals[row_id -1][col_id + 1]
                } else {
                    0
                };
            }
        }

        let mut top_3 = Top3::new();

        for top_row in 0 .. grid.len() {
            for top_col in 0 .. grid[0].len() {
                top_3.insert(grid[top_row][top_col]);

                for radius in 1 .. {
                    let mid_row = top_row + radius;
                    let (left_col, right_col) = (
                        top_col - radius, 
                        top_col + radius
                    );
                    let (bottom_row, bottom_col) = (
                        top_row + 2 * radius, 
                        top_col
                    );

                    if bottom_row >= grid.len() 
                    || left_col >= grid[0].len() 
                    || right_col >= grid[0].len() {
                        break;
                    }

                    let top_left = diagonals[mid_row][left_col] 
                    - if top_row > 0 && top_col + 1 < grid[0].len() {
                        diagonals[top_row - 1][top_col + 1]
                    } else {
                        0
                    };
                    let top_right = anti_diagonals[mid_row][right_col] 
                    - if top_row > 0 && top_col > 0 {
                        anti_diagonals[top_row - 1][top_col - 1]
                    } else {
                        0
                    };
                    let bottom_left = anti_diagonals[bottom_row][bottom_col] 
                    - if mid_row > 0 && left_col > 0 {
                        anti_diagonals[mid_row - 1][left_col - 1]
                    } else {
                        0
                    };
                    let bottom_right = diagonals[bottom_row][bottom_col] 
                    - if mid_row > 0 && right_col + 1 < grid[0].len() {
                        diagonals[mid_row - 1][right_col + 1]
                    } else {
                        0
                    };
                    
                    let corners = grid[top_row][top_col] 
                        + grid[mid_row][left_col] 
                        + grid[mid_row][right_col] 
                        + grid[bottom_row][bottom_col];
                    
                    top_3.insert(
                        top_left 
                        + top_right 
                        + bottom_left 
                        + bottom_right 
                        - corners
                    );
                }
            }
        }


        top_3.to_vec()
    }
}