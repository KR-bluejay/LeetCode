impl Solution {
    #[inline(always)]
    fn solve_child2(fruits: &Vec<Vec<i32>>, n: usize) -> i32 {
        let mut dp = vec![vec![i32::MIN / 2; n]; n];
        dp[0][n-1] = fruits[0][n-1];
        
        for i in 0..n-1 {
            for j in 0..n {
                if dp[i][j] == i32::MIN / 2 {
                    continue;
                }
                
                let cur = dp[i][j];
                
                // (i, j) -> (i+1, j-1)
                if j > 0 {
                    dp[i+1][j-1] = dp[i+1][j-1].max(cur + fruits[i+1][j-1]);
                }
                // (i, j) -> (i+1, j)
                dp[i+1][j] = dp[i+1][j].max(cur + fruits[i+1][j]);
                // (i, j) -> (i+1, j+1)
                if j + 1 < n {
                    dp[i+1][j+1] = dp[i+1][j+1].max(cur + fruits[i+1][j+1]);
                }
            }
        }
        
        dp[n-1][n-1]
    }
    
    fn solve_child3(fruits: &Vec<Vec<i32>>, n: usize) -> i32 {
        let mut dp = vec![vec![i32::MIN / 2; n]; n];
        dp[n-1][0] = fruits[n-1][0];
        
        for j in 0..n-1 {
            for i in 0..n {
                if dp[i][j] == i32::MIN / 2 {
                    continue;
                }
                
                let cur = dp[i][j];
                
                // (i, j) -> (i-1, j+1)
                if i > 0 {
                    dp[i-1][j+1] = dp[i-1][j+1].max(cur + fruits[i-1][j+1]);
                }
                // (i, j) -> (i, j+1)
                dp[i][j+1] = dp[i][j+1].max(cur + fruits[i][j+1]);
                // (i, j) -> (i+1, j+1)
                if i + 1 < n {
                    dp[i+1][j+1] = dp[i+1][j+1].max(cur + fruits[i+1][j+1]);
                }
            }
        }
        
        dp[n-1][n-1]
    }
    
    #[inline(always)]
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        let mut result = 0;
        
        // 1번 아이 (대각선)
        for i in 0..n {
            result += fruits[i][i];
            fruits[i][i] = 0;
        }
        
        // 2번 아이
        result += Self::solve_child2(&fruits, n);
        
        // 3번 아이
        result += Self::solve_child3(&fruits, n);
        
        result
    }
}