impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let k = k as usize;
        
        // dp[i][j][0] = max profit at day i with j transactions completed, not holding
        // dp[i][j][1] = max profit at day i with j transactions ongoing, holding (bought)
        // dp[i][j][2] = max profit at day i with j transactions ongoing, holding (shorted)
        
        let mut dp = vec![vec![vec![i64::MIN / 2; 3]; k + 1]; n + 1];
        
        // Initial state: day 0, 0 transactions, not holding
        for i in 0..=n {
            dp[i][0][0] = 0;
        }
        
        for i in 0..n {
            let price = prices[i] as i64;
            
            for j in 0..=k {
                // Not holding -> continue not holding
                dp[i + 1][j][0] = dp[i + 1][j][0].max(dp[i][j][0]);
                
                if j < k {
                    // Not holding -> buy (normal transaction)
                    dp[i + 1][j][1] = dp[i + 1][j][1].max(dp[i][j][0] - price);
                    
                    // Not holding -> short sell
                    dp[i + 1][j][2] = dp[i + 1][j][2].max(dp[i][j][0] + price);
                    
                    // Holding (bought) -> continue holding
                    dp[i + 1][j][1] = dp[i + 1][j][1].max(dp[i][j][1]);
                    
                    // Holding (shorted) -> continue holding
                    dp[i + 1][j][2] = dp[i + 1][j][2].max(dp[i][j][2]);
                    
                    // Holding (bought) -> sell (complete transaction)
                    dp[i + 1][j + 1][0] = dp[i + 1][j + 1][0].max(dp[i][j][1] + price);
                    
                    // Holding (shorted) -> buy back (complete transaction)
                    dp[i + 1][j + 1][0] = dp[i + 1][j + 1][0].max(dp[i][j][2] - price);
                }
            }
        }
        
        // Find maximum profit with at most k transactions
        let mut result = 0i64;
        for j in 0..=k {
            result = result.max(dp[n][j][0]);
        }
        
        result
    }
}