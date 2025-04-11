impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut symmetric_count = 0;

        for mid in low ..= high {
            if mid < 100 && mid % 11 == 0 {
                symmetric_count += 1;
                continue;
            } else if 1000 <= mid && mid < 10000 {
                let left = (mid / 1000) + (mid % 1000) / 100;
                let right = (mid % 100) / 10 + (mid % 10);
    
                if left == right {
                    symmetric_count += 1;
                }
            }
        }
        symmetric_count
    }
}