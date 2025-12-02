impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut results: Vec<i32> = vec![1; nums.len()];
        let mut cur = 1;
        let mut rev_cur = 1;

        for id in 0 .. nums.len() {
            let rev_id = nums.len() - 1 - id;
            
            results[id] *= cur;
            cur *= nums[id];
            
            results[rev_id] *= rev_cur;
            rev_cur *= nums[rev_id];
        }

        results
    }
}