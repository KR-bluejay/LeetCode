impl Solution {
    fn cleanup(i: usize, j: usize, board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
        visited[i][j] = true;

        if i + 1 < board.len() && board[i + 1][j] == 'O' && !visited[i + 1][j] {
            Self::cleanup(i + 1, j, board, visited);
        }

        if i > 0 && board[i - 1][j] == 'O' && !visited[i - 1][j] {
            Self::cleanup(i - 1, j, board, visited);
        }

        if j + 1 < board[0].len() && board[i][j + 1] == 'O' && !visited[i][j + 1] {
            Self::cleanup(i, j + 1, board, visited);
        }

        if j > 0 && board[i][j - 1] == 'O' && !visited[i][j - 1] {
            Self::cleanup(i, j - 1, board, visited);
        }
    }
    fn dfs(i: usize, j: usize, board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
        visited[i][j] = true;
        board[i][j] = 'X';

        if i + 1 < board.len() && board[i + 1][j] == 'O' && !visited[i + 1][j] {
            Self::dfs(i + 1, j, board, visited);
        }

        if i > 0 && board[i - 1][j] == 'O' && !visited[i - 1][j] {
            Self::dfs(i - 1, j, board, visited);
        }

        if j + 1 < board[0].len() && board[i][j + 1] == 'O' && !visited[i][j + 1] {
            Self::dfs(i, j + 1, board, visited);
        }

        if j > 0 && board[i][j - 1] == 'O' && !visited[i][j - 1] {
            Self::dfs(i, j - 1, board, visited);
        }
    }
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
        
        let m = board.len();
        let n = board[0].len();

        for i in 0 .. m {
            if board[i][0] == 'O' && !visited[i][0] {
                Self::cleanup(i, 0, board, &mut visited);
            }

            if n > 0 {
                if board[i][n - 1] == 'O' && !visited[i][n - 1] {
                    Self::cleanup(i, n - 1, board, &mut visited);
                }
            }
        } 
        for j in 0 .. n {
            if board[0][j] == 'O' && !visited[0][j] {
                Self::cleanup(0, j, board, &mut visited);
            }

            if m > 0 {
                if board[m - 1][j] == 'O' && !visited[m - 1][j] {
                    Self::cleanup(m - 1, j, board, &mut visited);
                }
            }
        }

        for i in 0 .. m {
            for j in 0 .. n {
                if board[i][j] == 'O' && !visited[i][j] {
                    Self::dfs(i, j, board, &mut visited);
                }
            }
        }
    }
}