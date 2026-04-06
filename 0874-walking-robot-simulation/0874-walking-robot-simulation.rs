use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        
        let (mut row_id, mut col_id) = (0, 0);
        let mut result = 0;
        let mut dir_id = 0;

        let mut obstacle_set: HashSet<(i32, i32)> = obstacles.into_iter()
            .map(|o| (o[1], o[0]))
            .collect();

        for command in commands.into_iter() {
            match command {
                -1 => {
                    dir_id = (dir_id + 1) % directions.len();
                },
                -2 => {
                    dir_id = (dir_id + 3) % directions.len();
                },
                command => {
                    let (dir_row, dir_col) = directions[dir_id];

                    for _ in 0 .. command {
                        let next_row_id = row_id + dir_row;
                        let next_col_id = col_id + dir_col;

                        if obstacle_set.contains(&(next_row_id, next_col_id)) {
                            break;
                        }
                        row_id = next_row_id;
                        col_id = next_col_id;

                        result = result.max(row_id.pow(2) + col_id.pow(2));
                    }
                },
            }
        }

        result
    }
}