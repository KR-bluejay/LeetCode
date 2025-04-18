impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut board: Vec<Vec<i32>> = vec![vec![0; n]; m];

        board[m - 1][n - 1] = 1;

        for i in (0 .. m).rev() {
            for j in (0 .. n).rev() {
                if i > 0 {
                    board[i - 1][j] += board[i][j];
                }

                if j > 0 {
                    board[i][j - 1] += board[i][j];
                }
            }
        }

        board[0][0]
    }
}