impl Solution {
    fn backtrack(
        m: usize, 
        n: usize, 
        cur_id: usize, 
        board: &Vec<Vec<char>>, 
        words: &Vec<char>, 
        visited: &mut Vec<Vec<bool>>
    ) -> bool {
        if cur_id == words.len() - 1 {
            return true;
        }

        let mut rtn = false;

        visited[m][n] = true;

        if n + 1 < board[m].len() && board[m][n + 1] == words[cur_id + 1] && !visited[m][n + 1] {
            rtn = rtn || Self::backtrack(m, n + 1, cur_id + 1, board, words, visited);
        }
        
        if 0 < n && board[m][n - 1] == words[cur_id + 1] && !visited[m][n - 1] {
            rtn = rtn ||Self::backtrack(m, n - 1, cur_id + 1, board, words, visited);
        }
        
        if m + 1 < board.len() && board[m + 1][n] == words[cur_id + 1] && !visited[m + 1][n]  {
            rtn = rtn || Self::backtrack(m + 1, n, cur_id + 1, board, words, visited);
        }
        
        if 0 < m && board[m - 1][n] == words[cur_id + 1] && !visited[m - 1][n] {
            rtn = rtn || Self::backtrack(m - 1, n, cur_id + 1, board, words, visited);
        }


        visited[m][n] = false;

        return rtn;
    }
    pub fn exist(board: Vec<Vec<char>>, mut word: String) -> bool {
        let words: Vec<char> = word.chars().collect();
        
        let mut visited: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
        let mut start_points: Vec<(usize, usize)> = Vec::new();

        for i in 0 .. board.len() {
            for j in 0 .. board[i].len() {
                if board[i][j] == words[0] {
                    start_points.push((i, j));
                }
            }
        }


        for (start_point_m, start_point_n) in start_points.iter() {
            if Self::backtrack(*start_point_m, *start_point_n, 0, &board, &words, &mut visited) {
                return true;
            }
        }
         

        false
    }
}